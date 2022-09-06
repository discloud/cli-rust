use serde::{Deserialize};

use super::FetchError;
#[derive(Deserialize)]
struct UserResponse {
    user: User
}
#[derive(Deserialize)]
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
}
pub fn fetch_user(token: String) -> Result<User, FetchError> {
    let client = reqwest::blocking::Client::new();
    let req = client.get(crate::api_url!("/user"))
        .header("api-token", token);
    match req.send() {
        Ok(res) => {
            if res.status().is_success() {
                Ok(res.json::<UserResponse>().unwrap().user)
            } else {
                Err(FetchError::APIReturnedError(res.status().as_u16()))
            }
        }
        Err(err) => Err(FetchError::FailedToConnect(err))
    }
}