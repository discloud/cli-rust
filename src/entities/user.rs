use serde::Deserialize;

use super::FetchError;

#[derive(Deserialize, Debug)]
pub struct TimeLeft {
    pub days: u32,
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
}
#[derive(Deserialize, Debug)]
pub struct User {
    #[serde(rename = "userID")]
    pub user_id: String,
    #[serde(rename = "totalRamMb")]
    pub total_ram_mb: u64,
    #[serde(rename = "ramUsedMb")]
    pub ram_used_mb: u64,
    #[serde(rename = "subdomains")]
    pub sub_domains: Vec<String>,
    #[serde(rename = "customdomains")]
    pub custom_domains: Vec<String>,
    pub apps: Vec<String>,
    pub plan: String,
    pub locale: String,
    #[serde(rename = "planDataEnd")]
    pub plan_data_end: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "lastDataLeft")]
    pub last_data_left: Option<TimeLeft>,
}
#[tracing::instrument]
pub fn fetch_user(token: String) -> Result<User, FetchError> {
    #[derive(Deserialize)]
    struct UserResponse {
        user: User,
        status: String,
        message: String,
    }
    let client = reqwest::blocking::Client::new();
    let req = client
        .get(crate::api_url!("/user"))
        .header("api-token", token);
    match req.send() {
        Ok(res) => {
            if res.status().is_success() {
                let res = res.json::<UserResponse>().unwrap();
                match res.status.as_str() {
                    "ok" => Ok(res.user),
                    "error" => Err(FetchError::FailedWithMessage(res.message)),
                    _ => unreachable!(),
                }
            } else {
                Err(FetchError::APIReturnedError(res.status().as_u16()))
            }
        }
        Err(err) => Err(FetchError::FailedToConnect(err)),
    }
}
