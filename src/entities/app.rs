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
            pub apps: App
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
    pub fn get_logs(token: String, id: u128) -> Result<String, FetchError> {
        #[derive(Deserialize)]
        struct Terminal {
            big: String
        }
        #[derive(Deserialize)]
        struct AppLogs {
            terminal: Terminal
        }
        #[derive(Deserialize)]
        struct LogsResponse {
            apps: Option<AppLogs>
        }
        let client = reqwest::blocking::Client::new();
        let req = client
            .get(crate::api_url!(format!("/app/{}/logs", id)))
            .header("api-token", token);
            match req.send() {
            Ok(res) => {
                if res.status().is_success() {
                    Ok(res.json::<LogsResponse>().unwrap().apps.unwrap().terminal.big)
                } else {
                    Err(FetchError::APIReturnedError(res.status().as_u16()))
                }
            }
            Err(err) => Err(FetchError::FailedToConnect(err)),
        }
    }
    pub fn restart(token: String, id: u128) -> Result<(), FetchError> {
        let client = reqwest::blocking::Client::new();
        let req = client
            .put(crate::api_url!(format!("/app/{}/restart", id)))
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
    pub fn start(token: String, id: u128) -> Result<(), FetchError> {
        let client = reqwest::blocking::Client::new();
        let req = client
            .put(crate::api_url!(format!("/app/{}/start", id)))
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
    pub fn stop(token: String, id: u128) -> Result<(), FetchError> {
        let client = reqwest::blocking::Client::new();
        let req = client
            .put(crate::api_url!(format!("/app/{}/stop", id)))
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
