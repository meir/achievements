
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

        let address = format!("localhost:{}", self.port);

        rouille::start_server(address, move |request| {
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

                _ => {
                    //get the file path from the request
                    let mut path = request.url().to_string();
                    println!("Request for file: {}", path);
                    // check if path is absolute
                    if path.starts_with("/") {
                        // remove the first character
                        path = ".".to_string() + path.as_str();
                    }

                    if path.contains("..") {
                        return Response::text("Invalid path").with_status_code(400);
                    }

                    let file = std::fs::read_to_string(&path).unwrap();
                    //send as mime type

                    let mime = mime_guess::from_path(&path).first_or_octet_stream();
                    Response::from_data(mime.to_string(), file)
                }
            )
        });
    }
}

