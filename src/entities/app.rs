use super::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct App {
    pub name: String,
    pub id: String,
    pub online: bool,
    #[serde(rename = "ramKilled")]
    pub ram_killed: bool,
    pub ram: u64,
    #[serde(rename = "mainFile")]
    pub main_file: String,
    pub lang: String,
}
impl App {
    pub fn fetch_all(token: String) -> Result<Vec<App>, FetchError> {
        #[derive(Deserialize)]
        struct AppsResponse {
            pub apps: Vec<App>,
        }
        let client = reqwest::blocking::Client::new();
        let req = client
            .get(crate::api_url!("/app/all"))
            .header("api-token", token);
        match req.send() {
            Ok(res) => {
                if res.status().is_success() {
                    Ok(res.json::<AppsResponse>().unwrap().apps)
                } else {
                    Err(FetchError::APIReturnedError(res.status().as_u16()))
                }
            }
            Err(err) => Err(FetchError::FailedToConnect(err)),
        }
    }
    
    pub fn fetch(token: String, id: u128) -> Result<App, FetchError> {
        #[derive(Deserialize)]
        struct AppResponse {
            pub apps: App,
        }
        let client = reqwest::blocking::Client::new();
        let req = client
            .get(crate::api_url!(format!("/app/{}", id)))
            .header("api-token", token);
        match req.send() {
            Ok(res) => {
                if res.status().is_success() {
                    Ok(res.json::<AppResponse>().unwrap().apps)
                } else {
                    Err(FetchError::APIReturnedError(res.status().as_u16()))
                }
            }
            Err(err) => Err(FetchError::FailedToConnect(err)),
        }
    }
    pub fn delete(token: String, id: u128) -> Result<(), FetchError> {
        let client = reqwest::blocking::Client::new();
        let req = client
            .delete(crate::api_url!(format!("/app/{}/delete", id)))
            .header("api-token", token);
            match req.send() {
            Ok(res) => {
                if res.status().is_success() {
                    Ok(())
                } else {
                    Err(FetchError::APIReturnedError(res.status().as_u16()))
                }
            }
            Err(err) => Err(FetchError::FailedToConnect(err)),
        }
    }
}
