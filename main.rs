mod steam;
mod twitch;
mod http;

use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::{path::Path, fs::File, io::Write};
use dotenv::dotenv;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use tray_item::TrayItem;

fn main() {
    create_env_file();
    create_css_file();
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
    let twitch = twitch::TwitchAPI::new(twitch_api_key.as_str(), twitch_client_id.as_str());
    let mut appstate = http::AppState::new(username, twitch, steam);
    appstate.load_achievements();
    let state = Arc::new(Mutex::new(appstate));
    
    let mut tray = TrayItem::new("Achievements", "").unwrap();
    let refstate = RefCell::new(Arc::clone(&state));
    tray.add_menu_item("Reload", move || {
        let state = refstate.borrow_mut();
        state.lock().unwrap().load_achievements();
    }).unwrap();
    
    tray.add_menu_item("Copy URL", move || {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        let url = format!("http://localhost:{}", port);
        ctx.set_contents(url).unwrap();
    }).unwrap();
    
    tray.add_menu_item("Open in browser", move || {
        let url = format!("http://localhost:{}", port);
        open::that(url).unwrap();
    }).unwrap();
    
    tray.add_menu_item("Quit", move || {
        std::process::exit(0);
    }).unwrap();
    
    std::thread::spawn(move || {
        println!("Starting server... ");
        let mut http = http::WebServer::new(port, state);
        http.start();
    });

    let inner = tray.inner_mut();
    inner.display();
}

fn create_env_file() {
    //check if .env exists
    let path = Path::new(".env");
    if path.exists() {
        return;
    }

    //create .env file
    let mut file = File::create(path).unwrap();
    let defaults = include_str!("./example.env");
    let result = file.write(defaults.as_bytes());
    if result.is_err() {
        println!("Failed to create .env file");
        std::process::exit(1);
    }

    println!("Created .env file, please fill in the values");
    std::process::exit(0);
}

fn create_css_file() {
    //check if custom.css exists
    let path = Path::new("custom.css");
    if path.exists() {
        return;
    }

    //create custom.css file
    let mut file = File::create(path).unwrap();
    let defaults = include_str!("./example.css");
    let result = file.write(defaults.as_bytes());
    if result.is_err() {
        println!("Failed to create custom.css file");
        std::process::exit(1);
    }

    println!("Created custom.css file, please fill in the values");
    std::process::exit(0);
}

