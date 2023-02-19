use std::collections::HashMap;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct SteamAPI {
    api_key: String,
    user_id: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct GameAchievements {
    pub app_id: u64,
    pub achievements: HashMap<String, Achievement>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Achievement {
    pub name: String,
    pub description: String,
    pub icon: String,
    pub icon_gray: String,
    pub unlocked: bool,
}
const URL_GET_APP_ID: &str = "https://api.steampowered.com/IPlayerService/GetRecentlyPlayedGames/v1";
const URL_GET_USER_ACHIEVEMENTS: &str = "https://api.steampowered.com/ISteamUserStats/GetPlayerAchievements/v1/";
const URL_GET_ACHIEVEMENT_SCHEMA: &str = "https://api.steampowered.com/ISteamUserStats/GetSchemaForGame/v2/";

#[allow(dead_code)]
impl SteamAPI {
    pub fn new(api_key: &str, user_id: &str) -> Self {
        Self { 
            api_key: api_key.to_string(),
            user_id: user_id.to_string(),
        }
    }

    pub fn get_appid(&self, app_name: &str) -> Result<u64, ureq::Error> {
        let body: String = ureq::get(URL_GET_APP_ID)
            .query("key", &self.api_key)
            .query("steamid", &self.user_id)
            .call()?
            .into_string()?;

        let json: serde_json::Value = serde_json::from_str(&body).unwrap();

        let apps = json["response"]["games"].as_array().unwrap();

        for app in apps {
            let name = app["name"].as_str().unwrap();

            if name == app_name {
                let id = app["appid"].as_u64().unwrap();

                return Ok(id)
            }
        }

        Ok(0)
    }

    pub fn get_achievements(&self, app_id: u64) -> Result<GameAchievements, ureq::Error> {
        let mut game_achievement = GameAchievements {
            app_id: app_id,
            achievements: HashMap::new(),
        };

        {
            let body: String = ureq::get(URL_GET_ACHIEVEMENT_SCHEMA)
            .query("key", &self.api_key)
            .query("steamid", &self.user_id)
            .query("appid", &app_id.to_string())
            .call()?
            .into_string()?;

            let json: serde_json::Value = serde_json::from_str(&body).unwrap();

            let achievements = json["game"]["availableGameStats"]["achievements"].as_array().unwrap();

            for achievement in achievements {
                let name = achievement["name"].as_str().unwrap().to_string();
                let display_name = achievement["displayName"].as_str().unwrap().to_string();
                let description = achievement["description"].as_str().unwrap().to_string();
                let icon = achievement["icon"].as_str().unwrap().to_string();
                let icon_gray = achievement["icongray"].as_str().unwrap().to_string();

                game_achievement.achievements.insert(name, Achievement {
                    name: display_name,
                    description: description,
                    icon: icon,
                    icon_gray,
                    unlocked: false,
                });
            }
        }

        {
            let body: String = ureq::get(URL_GET_USER_ACHIEVEMENTS)
            .query("key", &self.api_key)
            .query("steamid", &self.user_id)
            .query("appid", &app_id.to_string())
            .call()?
            .into_string()?;

            let json: serde_json::Value = serde_json::from_str(&body).unwrap();

            let achievements = json["playerstats"]["achievements"].as_array().unwrap();

            for achievement in achievements {
                let name = achievement["apiname"].as_str().unwrap().to_string();
                let unlocked = achievement["achieved"] == 1;

                let achievement = game_achievement.achievements.get_mut(&name).unwrap();

                achievement.unlocked = unlocked;
            }
        }

        return Ok(game_achievement)
    }

    pub fn update_achievements(&self, mut game_achievement: GameAchievements) -> Result<GameAchievements, ureq::Error> {
        let body: String = ureq::get(URL_GET_USER_ACHIEVEMENTS)
        .query("key", &self.api_key)
        .query("steamid", &self.user_id)
        .query("appid", &game_achievement.app_id.to_string())
        .call()?
        .into_string()?;

        let json: serde_json::Value = serde_json::from_str(&body).unwrap();

        let achievements = json["playerstats"]["achievements"].as_array().unwrap();

        for achievement in achievements {
            let name = achievement["apiname"].as_str().unwrap().to_string();
            let unlocked = achievement["achieved"] == 1;

            let achievement = game_achievement.achievements.get_mut(&name).unwrap();

            achievement.unlocked = unlocked;
        }

        return Ok(game_achievement)
    }
}

