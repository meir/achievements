
use rouille::{Response, router};

use crate::steam::SteamAPI;
use crate::steam::GameAchievements;

pub struct WebServer {
    port: u16,
    appstate: AppState,
}

#[derive(Clone)]
pub struct AppState {
    achievements: GameAchievements,
    steam_api: SteamAPI,
}

impl AppState {
    pub fn new(achievements: GameAchievements, steam_api: SteamAPI) -> AppState {
        // Initialize your data here
        AppState {
            achievements: achievements,
            steam_api: steam_api,
        }
    }
}


impl WebServer {
    pub fn new(port: u16, appstate: AppState) -> Self {
        Self { 
            port: port,
            appstate: appstate,
        }
    }

    pub fn start(&mut self) {
        println!("Server started on port {}", self.port);

        let app_state = self.appstate.clone();

        rouille::start_server("0.0.0.0:80", move |request| {
            router!(request,
                (GET) (/) => {
                    let file = std::fs::read_to_string("index.html").unwrap();
                    Response::html(file)
                },

                (GET) (/update) => {
                    let updated_achievements = app_state.steam_api.update_achievements(app_state.achievements.clone()).unwrap();
                    let json = serde_json::to_string(&updated_achievements).unwrap();
                    Response::text(json)
                },

                _ => Response::text("404")
            )
        });
    }
}

