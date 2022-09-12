use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use serde_enum_str::*;

use super::{app::App, user::User, FetchError};
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
    Backup,
}
impl Debug for Feature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Backup => "backup",
            Self::Commit => "commit",
            Self::Restart => "restart",
            Self::SeeLogs => "logs",
            Self::SetRam => "ram",
            Self::Start => "start",
            Self::Status => "status",
            Self::Stop => "stop",
        })
    }
}
#[derive(Deserialize, Serialize, Clone)]
pub struct Mod<'a> {
    #[serde(rename = "modID")]
    user_id: u128,
    #[serde(rename = "perms")]
    features: Vec<Feature>,
    #[serde(skip)]
    app: Option<&'a App>,
}
impl<'a> Mod<'a> {
    pub fn new(
        token: String,
        user: &User,
        app: &'a App,
        features: Vec<Feature>,
    ) -> Result<Mod<'a>, FetchError> {
        let moderator = Self {
            user_id: user.user_id.parse().unwrap(),
            features,
            app: Some(app),
        };
        moderator.add(token)?;
        Ok(moderator)
    }
    pub fn fetch_mod(
        token: String,
        user: &User,
        app: &'a App,
    ) -> Result<Option<Mod<'a>>, FetchError> {
        #[derive(Deserialize)]
        struct Response<'a> {
            status: String,
            message: Option<String>,
            team: Option<Vec<Mod<'a>>>,
        }
        let client = reqwest::blocking::Client::new();
        let req = client
            .get(crate::api_url!(format!("/app/{}/team", app.id)))
            .header("api-token", token);
        match req.send() {
            Ok(res) => match res.json::<Response>() {
                Err(err) => Err(FetchError::FailedWithMessage(err.to_string())),
                Ok(response) => {
                    if response.status == "ok" {
                        let moderator = response
                            .team
                            .unwrap()
                            .iter()
                            .find(|m| m.user_id.to_string() == user.user_id)
                            .map(|m| Self {
                                app: Some(app),
                                ..m.clone()
                            });
                        Ok(moderator)
                    } else {
                        Err(FetchError::FailedWithMessage(response.message.unwrap()))
                    }
                }
            },
            Err(err) => Err(FetchError::FailedToConnect(err)),
        }
    }
    pub fn get_features(&self) -> Vec<Feature> {
        self.features.clone()
    }
    /// Returns the app this moderator is at
    /// # Panics
    /// This will panic if `app` is **None**
    pub fn get_app(&'a self) -> &'a App {
        self.app.unwrap()
    }
    /// Adds this moderator to the app
    /// # Panics
    /// This will panic if `app` is **None**
    pub fn add(&'a self, token: String) -> Result<(), FetchError> {
        #[derive(Deserialize)]
        struct Response {
            status: String,
            message: Option<String>,
        }
        let client = reqwest::blocking::Client::new();
        let req = client
            .post(crate::api_url!(format!("/app/{}/team", self.get_app().id)))
            .header("api-token", token)
            .json(self);
        match req.send() {
            Ok(res) => match res.json::<Response>() {
                Err(err) => Err(FetchError::FailedWithMessage(err.to_string())),
                Ok(response) => {
                    if response.status == "ok" {
                        Ok(())
                    } else {
                        Err(FetchError::FailedWithMessage(response.message.unwrap()))
                    }
                }
            },
            Err(err) => Err(FetchError::FailedToConnect(err)),
        }
    }

    pub fn remove(self, token: String) -> Result<(), FetchError> {
        #[derive(Deserialize)]
        struct Response {
            status: String,
            message: Option<String>,
        }
        let client = reqwest::blocking::Client::new();
        let req = client
            .delete(crate::api_url!(format!(
                "/app/{}/team/{}",
                self.get_app().id,
                self.user_id
            )))
            .header("api-token", token);
        match req.send() {
            Ok(res) => match res.json::<Response>() {
                Err(err) => Err(FetchError::FailedWithMessage(err.to_string())),
                Ok(response) => {
                    if response.status == "ok" {
                        Ok(())
                    } else {
                        Err(FetchError::FailedWithMessage(response.message.unwrap()))
                    }
                }
            },
            Err(err) => Err(FetchError::FailedToConnect(err)),
        }
    }
}
