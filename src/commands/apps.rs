use colored::Colorize;

use crate::entities::FetchError;
#[tracing::instrument]
pub fn apps(teams: bool) {
    let token = super::expect_token();
    match if !teams {crate::entities::app::App::fetch_all(token)} else {crate::entities::app::App::fetch_foreign_apps(token)} {
        Ok(apps) => {
            println!("{}Your apps:", if teams{"(Not) "} else {""});
            for app in apps {
                println!(
                    "- {} (lang: {}, id: {}, Online: {})",
                    app.name.green(),
                    app.lang.yellow(),
                    app.id.to_string().bright_black(),
                    
                    if app.online {"yes".green().bold()} else {"no".bright_black().bold()}
                );
            }
        }
        Err(err) => match err {
            FetchError::APIReturnedError(code) => match code {
                404 => super::err("You don't have any apps. Use `discloud up` to upload one."),
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
