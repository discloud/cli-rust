use crate::auth;
use std::io::ErrorKind;
#[tracing::instrument]
pub fn authstatus() -> std::io::Result<()> {
    match auth::get_token() {
        Ok(token) => {
            super::log("You're already logged in!\n");
            super::log(&format!("Token: {}{}", "*".repeat(token.len() - 16), &token[token.len()-16..]));
            super::check_token();
        }
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                super::err("You're not logged in yet!");
            }
            err => {
                super::err(&format!("Couldn't open token file: {}", err));
            }
        },
    }
    Ok(())
}
