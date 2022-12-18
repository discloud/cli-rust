pub mod moderator;
use std::fmt::Display;

pub mod app;
pub mod user;
#[derive(Debug)]
pub enum FetchError {
    NotLoggedIn,
    FailedToConnect(reqwest::Error),
    APIReturnedError(u16),
    FailedWithMessage(String),
}
impl Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotLoggedIn => f.write_str("not logged in"),
            Self::FailedToConnect(err) => f.write_str(&format!("failed to connect: {}", err)),
            Self::APIReturnedError(status) => {
                f.write_str(&format!("api returned code: {}", status))
            }
            Self::FailedWithMessage(msg) => f.write_str(msg),
        }
    }
}
