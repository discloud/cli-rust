use super::FetchError;
use serde::{Deserialize, Serialize};
use serde_enum_str::*;
use std::fmt::Debug;

#[derive(Deserialize_enum_str, Serialize_enum_str, Clone, Eq, PartialEq)]
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
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Mod {
    #[serde(rename = "modID")]
    user_id: String,
    #[serde(rename = "perms")]
    features: Vec<Feature>,
    #[serde(skip)]
    app_id: u128,
}
impl Mod {
    pub fn new(
        token: String,
        user_id: u128,
        app_id: u128,
        features: Vec<Feature>,
    ) -> Result<Mod, FetchError> {
        let moderator = Self {
            user_id: user_id.to_string(),
            features,
            app_id,
        };
        moderator.add(token)?;
        Ok(moderator)
    }

    pub fn id(&self) -> u128 {
        self.user_id.parse().unwrap()
    }
    pub fn set_features(
        &mut self,
        permissions: Vec<Feature>,
        token: String,
    ) -> Result<(), FetchError> {
        self.features = permissions;
        #[derive(Deserialize)]
        struct Response {
            status: String,
            message: Option<String>,
        }
        let client = reqwest::blocking::Client::new();
        let req = client
            .put(crate::api_url!(format!("/app/{}/team", self.app_id)))
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
    pub fn fetch_mod(
        token: String,
        user_id: u128,
        app_id: u128,
    ) -> Result<Option<Mod>, FetchError> {
        #[derive(Deserialize)]
        struct Response {
            status: String,
            message: Option<String>,
            team: Option<Vec<Mod>>,
        }
        let client = reqwest::blocking::Client::new();
        let req = client
            .get(crate::api_url!(format!("/app/{}/team", app_id)))
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
                            .find(|m| m.user_id == user_id.to_string())
                            .map(|m| Self {
                                app_id,
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
    #[tracing::instrument]
    pub fn get_features(&self) -> Vec<Feature> {
        self.features.clone()
    }
    /// Adds this moderator to the app
    pub fn add(&self, token: String) -> Result<(), FetchError> {
        #[derive(Deserialize)]
        struct Response {
            status: String,
            message: Option<String>,
        }
        let client = reqwest::blocking::Client::new();
        let req = client
            .post(crate::api_url!(format!("/app/{}/team", self.app_id)))
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
                self.app_id, self.user_id
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
