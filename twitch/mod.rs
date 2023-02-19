
#[derive(Clone)]
pub struct TwitchAPI {
    client_secret: String,
    client_id: String,
    token: String,
}

pub struct TwitchStreamer {
    api: TwitchAPI,
    id: String,
}

const URL_AUTHORIZE: &str = "https://id.twitch.tv/oauth2/token";
const URL_GET_USER: &str = "https://api.twitch.tv/helix/users";
const URL_GET_STREAM_INFO: &str = "https://api.twitch.tv/helix/channels";

/*

curl -X POST 'https://id.twitch.tv/oauth2/token' \
-H 'Content-Type: application/x-www-form-urlencoded' \
-d 'client_id={client_id}&client_secret={client_secret}&grant_type=client_credentials'

 */

impl TwitchAPI {
    pub fn new(client_secret: &str, client_id: &str) -> Self {
        Self { 
            client_secret: client_secret.to_string(),
            client_id: client_id.to_string(),
            token: "".to_string(),
        }
    }

    pub fn authorize(&mut self) -> Result<(), ureq::Error> {
        let body: String = ureq::post(URL_AUTHORIZE)
            .query("client_id", self.client_id.as_str())
            .query("client_secret", self.client_secret.as_str())
            .query("grant_type", "client_credentials")
            .call()?
            .into_string()?;

        let json: serde_json::Value = serde_json::from_str(&body).unwrap();

        let mut token = "Bearer ".to_string();
        token.push_str(json["access_token"].as_str().unwrap());
        self.token = token.to_string();

        return Ok(())
    }

    pub fn get_user_id(&self, username: &str) -> Result<TwitchStreamer, ureq::Error> {
        let body: String = ureq::get(URL_GET_USER)
            .query("login", username)
            .set("Authorization", self.token.as_str())
            .set("Client-Id", self.client_id.as_str())
            .call()?
            .into_string()?;

        let json: serde_json::Value = serde_json::from_str(&body).unwrap();

        let id = json["data"][0]["id"].as_str();

        return Ok(TwitchStreamer {
            api: self.clone(),
            id: id.unwrap().to_string(),
        })
    }
}

impl TwitchStreamer {
    pub fn get_current_game(&self) -> Result<String, ureq::Error> {
        let body: String = ureq::get(URL_GET_STREAM_INFO)
            .query("broadcaster_id", &self.id)
            .set("Authorization", self.api.token.as_str())
            .set("Client-Id", self.api.client_id.as_str())
            .call()?
            .into_string()?;

        let json: serde_json::Value = serde_json::from_str(&body).unwrap();

        let game = json["data"][0]["game_name"].as_str().unwrap().to_string();

        return Ok(game)
    }
}