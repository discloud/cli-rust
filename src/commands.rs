pub mod aboutme;
pub mod apps;
pub mod authstatus;
pub mod commit;
pub mod status;
pub mod init;
pub mod login;
pub mod logs;
pub mod mods;
pub mod remove;
pub mod restart;
pub mod start;
pub mod stop;
pub mod upload;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};
use spinners::*;
#[macro_export]
macro_rules! handle_result {
    ($v:expr) => {
        match $v {
            Ok(v) => v,
            Err(err) => {
                super::err(&err.to_string());
                std::process::exit(1);
            }
        }
    };
    ($v:expr, $spinner:ident) => {
        match $v {
            Ok(v) => v,
            Err(err) => {
                $spinner.stop_with_message(super::format_err(&err.to_string()));
                std::process::exit(1);
            }
        }
    };
}
use crate::entities::{FetchError, app::App};
#[tracing::instrument]
pub fn expect_token() -> String {
    if crate::auth::validate_token() {
        log("Your token is valid!");
        crate::auth::get_token().unwrap()
    } else {
        err("Your token is invalid!");
        std::process::exit(1);
    }
}
pub fn check_token() {
    let mut validate_spinner = Spinner::new(Spinners::Dots12, "Checking token".into());
    validate_spinner.stop_with_message(if crate::auth::validate_token() {
        format_log("Your token is valid!")
    } else {
        format_err("Your token is invalid!")
    });
}
pub fn format_log(msg: &str) -> String {
    format!("{} {}", "✔".green().bold(), msg)
}
pub fn format_warn(msg: &str) -> String {
    format!("{} {}", "!".yellow().bold(), msg)
}
pub fn format_err(msg: &str) -> String {
    format!("{} {}", "✘".red().bold(), msg)
}
pub fn log(msg: &str) {
    println!("{}", format_log(msg));
}

pub fn warn(msg: &str) {
    println!("{}", format_warn(msg));
}

pub fn err(msg: &str) {
    println!("{}", format_err(msg));
}
#[cfg(test)]
mod tests {
    use colored::Colorize;

    #[test]
    fn log() {
        let mut out = "✔".green().bold().to_string();
        out.push_str(" Some logs");
        assert_eq!(super::format_log("Some logs"), out)
    }

    #[test]
    fn err() {
        let mut out = "X".red().bold().to_string();
        out.push_str(" Some errors");
        assert_eq!(super::format_err("Some errors"), out)
    }

    #[test]
    fn warn() {
        let mut out = "!".yellow().bold().to_string();
        out.push_str(" Some warnings");
        assert_eq!(super::format_warn("Some warnings"), out)
    }
}

pub fn ask_for_app(token: String, action: &str, teams: bool) -> Result<App, FetchError> {
    let mut apps = if teams {
        crate::entities::app::App::fetch_foreign_apps(token)
    } else {
        crate::entities::app::App::fetch_all(token)
    }?;
    match apps.len() {
        0 => {
            err("You don't have any apps!");
            std::process::exit(1);
        },
        1 => Ok(apps.remove(0)),
        _ => {
            let options = apps
                .iter()
                .map(|app| format!("{}: ({}) {}", app.name, app.lang, app.id))
                .collect::<Vec<_>>();
            let chosen_opt = Select::with_theme(&ColorfulTheme::default())
                .items(&options)
                .with_prompt(format!("Which app you want to {}?", action))
                .interact()
                .unwrap();
            Ok(apps.remove(chosen_opt))
        }
    }
}

pub fn ask_for_app_id(token: String, action: &str, teams: bool) -> Result<u128, FetchError> {
    let apps = if teams {
        crate::entities::app::App::fetch_foreign_apps(token)
    } else {
        crate::entities::app::App::fetch_all(token)
    }?;
    match apps.len() {
        0 => {
            err("You don't have any apps!");
            std::process::exit(1);
        },
        1 => Ok(apps[0].id.parse().unwrap()),
        _ => {
            let options = apps
                .iter()
                .map(|app| format!("{}: ({}) {}", app.name, app.lang, app.id))
                .collect::<Vec<_>>();
            let chosen_opt = Select::with_theme(&ColorfulTheme::default())
                .items(&options)
                .with_prompt(format!("Which app you want to {}?", action))
                .interact()
                .unwrap();
            Ok(apps[chosen_opt].id.parse().unwrap())
        }
    }
}
