use super::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct App {
    pub name: String,
    pub id: String,
    pub online: bool,
    pub lang: String,
}
impl App {
    // Fetches apps from /team instead of /app/all
    pub fn fetch_foreign_apps(token: String) -> Result<Vec<App>, FetchError> {
        #[derive(Deserialize)]
        struct AppsResponse {
            pub apps: Vec<App>,
        }
        let client = reqwest::blocking::Client::new();
        let req = client
            .get(crate::api_url!("/team"))
            .header("api-token", token);
        match req.send() {
            Ok(res) => Ok(res.json::<AppsResponse>().unwrap().apps),
            Err(err) => Err(FetchError::FailedToConnect(err)),
        }
    }
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
    pub fn get_logs(token: String, id: u128, team: bool) -> Result<String, FetchError> {
        #[derive(Deserialize)]
        struct Terminal {
            big: String,
        }
        #[derive(Deserialize)]
        struct AppLogs {
            terminal: Terminal,
        }
        #[derive(Deserialize)]
        struct LogsResponse {
            apps: Option<AppLogs>,
        }
        let client = reqwest::blocking::Client::new();
        let req = client
            .get(crate::api_url!(format!("/{}/{}/logs", if team {"team"} else {"app"}, id)))
            .header("api-token", token);
        match req.send() {
            Ok(res) => {
                if res.status().is_success() {
                    Ok(res
                        .json::<LogsResponse>()
                        .unwrap()
                        .apps
                        .unwrap()
                        .terminal
                        .big)
                } else {
                    Err(FetchError::APIReturnedError(res.status().as_u16()))
                }
            }
            Err(err) => Err(FetchError::FailedToConnect(err)),
        }
    }
    pub fn restart(token: String, id: u128, team: bool) -> Result<(), FetchError> {
        let client = reqwest::blocking::Client::new();
        let req = client
            .put(crate::api_url!(format!("/{}/{}/restart", if team {"team"} else {"app"}, id)))
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
    pub fn start(token: String, id: u128, team: bool) -> Result<(), FetchError> {
        let client = reqwest::blocking::Client::new();
        let req = client
            .put(crate::api_url!(format!("/{}/{}/start", if team {"team"} else {"app"}, id)))
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
    pub fn stop(token: String, id: u128, team: bool) -> Result<(), FetchError> {
        let client = reqwest::blocking::Client::new();
        let req = client
            .put(crate::api_url!(format!("/{}/{}/stop", if team {"team"} else {"app"}, id)))
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
