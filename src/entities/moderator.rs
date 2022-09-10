use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use serde_enum_str::*;

use super::{user::User, app::App, FetchError};
#[derive(Deserialize_enum_str, Serialize_enum_str, Clone)]
pub enum Feature {
    #[serde(rename = "start_app")]
    Start,
    #[serde(rename = "stop_app")]
    Stop,
    #[serde(rename = "restart_app")]
    Restart,
    #[serde(rename = "logs_app")]
    SeeLogs,
    #[serde(rename = "commit_app")]
    Commit,
    #[serde(rename = "status_app")]
    Status,
    #[serde(rename = "edit_ram")]
    SetRam,
    #[serde(rename = "backup_app")]
    Backup
}
impl Debug for Feature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                Self::Backup => "backup",
                Self::Commit => "commit",
                Self::Restart => "restart",
                Self::SeeLogs => "logs",
                Self::SetRam => "ram",
                Self::Start => "start",
                Self::Status => "status",
                Self::Stop => "stop"
            }
        )
    }
}
#[derive(Deserialize, Serialize)]
pub struct Mod {
    #[serde(rename = "mod_id")]
    user_id: u128,
    #[serde(rename = "perms")]
    features: Vec<Feature>
}
impl Mod {
    pub fn new(token: String, user: &User, app: &App, features: Vec<Feature>) -> Result<Mod, FetchError> {
        let moderator = Self {
            user_id: user.user_id.parse().unwrap(),
            features
        };
        moderator.add_to(token, app)?;
        Ok(moderator)
    }
    pub fn get_features(&self) -> Vec<Feature> {
        self.features.clone()
    }
    pub fn add_to(&self, token: String, app: &App) -> Result<(), FetchError>{
        #[derive(Deserialize)]
        struct Response {
            status: String,
            message: Option<String>
        }
        let client = reqwest::blocking::Client::new();
        let req = client
            .post(crate::api_url!(format!("/app/{}/team", app.id)))
            .header("api-token", token)
            .json(self);
        match req.send() {
            Ok(res) => {
                match res.json::<Response>() {
                    Err(err) => {
                        Err(FetchError::FailedWithMessage(err.to_string()))
                    }
                    Ok(response) => {
                        if response.status == "ok" {
                            Ok(())
                        } else {
                            Err(FetchError::FailedWithMessage(response.message.unwrap()))
                        }
                    }
                }
                
            }
            Err(err) => Err(FetchError::FailedToConnect(err)),
        }
    }
}