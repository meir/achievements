
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use rouille::{Response, router};

use crate::twitch::TwitchAPI;
use crate::steam::SteamAPI;
use crate::steam::GameAchievements;

pub struct WebServer {
    port: u16,
    appstate: Arc<Mutex<AppState>>
}

#[derive(Clone)]
pub struct AppState {
    pub achievements: GameAchievements,
    pub username: String,
    pub twitch_api: TwitchAPI,
    pub steam_api: SteamAPI,
}

impl AppState {
    pub fn new(username: String, twitch_api: TwitchAPI, steam_api: SteamAPI) -> AppState {
        // Initialize your data here
        AppState {
            achievements: GameAchievements {
                app_id: 0,
                achievements: HashMap::new(),
            },
            username: username,
            twitch_api: twitch_api,
            steam_api: steam_api,
        }
    }

    pub fn load_achievements(&mut self) {
        println!("Authorizing...");
        self.twitch_api.authorize().unwrap();
        
        println!("Getting user id... ");
        let user = self.twitch_api.get_user_id(self.username.as_str()).unwrap();
        
        println!("Getting current game... ");
        let game = user.get_current_game().unwrap();
        
        println!("Getting appid... ");
        let id = self.steam_api.get_appid(&game).unwrap();
        
        println!("{:?}", id);
        
        println!("Getting achievements... ");
        self.achievements = self.steam_api.get_achievements(id).unwrap();
    }
}


impl WebServer {
    pub fn new(port: u16, appstate: Arc<Mutex<AppState>>) -> Self {
        Self { 
            port: port,
            appstate: appstate
        }
    }

    pub fn start(&mut self) {
        println!("Server started on port {}", self.port);

        let app_state = self.appstate.clone();

        let address = format!("localhost:{}", self.port);

        rouille::start_server(address, move |request| {
            router!(request,
                (GET) (/) => {
                    let file = include_str!("./index.html");
                    Response::html(file)
                },

                (GET) (/update) => {
                    let updated_achievements = {
                        let mut state = app_state.lock().unwrap();
                        state.load_achievements();
                        state.achievements.clone()
                    };
                    let json = serde_json::to_string(&updated_achievements).unwrap();
                    Response::text(json)
                },

                (GET) (/custom) => {
                    let file = std::fs::read_to_string("custom.css").unwrap();
                    Response::from_data("text/css", file)
                },

                _ => Response::empty_404()
            )
        });
    }
}

