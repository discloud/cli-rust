use colored::Colorize;

use crate::entities::FetchError;
#[tracing::instrument]
pub fn apps() {
    let token = super::expect_token();
    match crate::entities::app::App::fetch_foreign_apps(token) {
        Ok(apps) => {
            println!("(Not) Your apps:");
            for app in apps {
                println!(
                    "- {}: ({}) {}",
                    app.name.green(),
                    app.lang.yellow(),
                    app.id.to_string().bright_black()
                );
            }
        }
        Err(err) => match err {
            FetchError::APIReturnedError(code) => match code {
                404 => super::err("No one gave you permission to do stuff on their apps."),
                _ => {
                    super::err(&err.to_string());
                }
            },
            err => {
                super::err(&err.to_string());
            }
        },
    }
}
