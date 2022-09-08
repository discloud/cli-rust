use colored::Colorize;

use crate::entities::FetchError;
pub fn apps() {
    let token = super::expect_token();
    match crate::entities::app::App::fetch_all(token.clone()) {
        Ok(apps) => {
            println!("Your apps:");
            for app in apps {
                println!("- {}: ({}) {}", app.name.green(), app.lang.yellow(), app.id.to_string().bright_black());
            }
        }
        Err(err) => {
            match err {
                FetchError::APIReturnedError(code) =>{
                    match code {
                        404 => {
                            super::err("You don't have any apps. Use `discloud up` to upload one.")
                        }
                        _ => {
                            super::err(&err.to_string());
                        }
                    }
                }
                err => {
                    super::err(&err.to_string());
                }
            }
        }
    }
}