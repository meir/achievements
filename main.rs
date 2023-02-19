mod steam;
mod twitch;
mod http;

use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let steam_api_key = std::env::var("STEAM_API_KEY").unwrap();
    let steam_user_id = std::env::var("STEAM_USER_ID").unwrap();
    let twitch_api_key = std::env::var("TWITCH_API_KEY").unwrap();
    let twitch_client_id = std::env::var("TWITCH_CLIENT_ID").unwrap();
    let username = std::env::var("TWITCH_USERNAME").unwrap();
    let string_port = std::env::var("PORT");
    if string_port.is_err() {
        println!("PORT not set, using default port 8080");
    }

    let mut port = 8080;
    if string_port.is_ok() {
        port = string_port.unwrap().parse::<u16>().unwrap();
    }

    let steam = steam::SteamAPI::new(steam_api_key.as_str(), steam_user_id.as_str());
    let mut twitch = twitch::TwitchAPI::new(twitch_api_key.as_str(), twitch_client_id.as_str());
    
    println!("Authorizing...");
    twitch.authorize().unwrap();
    
    println!("Getting user id... ");
    let user = twitch.get_user_id(username.as_str()).unwrap();
    
    println!("Getting current game... ");
    let game = user.get_current_game().unwrap();
    
    println!("Getting appid... ");
    let id = steam.get_appid(&game).unwrap();
    
    println!("{:?}", id);
    
    println!("Getting achievements... ");
    let achievements = steam.get_achievements(id).unwrap();

    let appstate = http::AppState::new(achievements, steam);
    let mut http = http::WebServer::new(port, appstate);

    println!("Starting server... ");
    http.start();
}