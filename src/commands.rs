pub mod aboutme;
pub mod apps;
pub mod authstatus;
pub mod commit;
pub mod init;
pub mod login;
pub mod logs;
pub mod mods;
pub mod remove;
pub mod restart;
pub mod start;
pub mod stop;
pub mod teams;
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
use crate::entities::FetchError;
#[tracing::instrument]
pub fn expect_token() -> String {
    if crate::auth::validate_token() {
        log("Your token is valid!");
        return crate::auth::get_token().unwrap();
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
        let mut out = String::from("✔".green().bold().to_string());
        out.push_str(" Some logs");
        assert_eq!(super::format_log("Some logs"), out)
    }

    #[test]
    fn err() {
        let mut out = String::from("✘".red().bold().to_string());
        out.push_str(" Some errors");
        assert_eq!(super::format_err("Some errors"), out)
    }

    #[test]
    fn warn() {
        let mut out = String::from("!".yellow().bold().to_string());
        out.push_str(" Some warnings");
        assert_eq!(super::format_warn("Some warnings"), out)
    }
}
pub fn ask_for_app(token: String, action: &str) -> Result<u128, FetchError> {
    let user = crate::entities::user::fetch_user(token.clone())?;
    match user.apps.len() {
        0 => {
            err("You don't have any apps. Use `discloud up` to upload one.");
            std::process::exit(1)
        }
        1 => Ok(user.apps[0].parse().unwrap()),
        _ => {
            let apps = crate::entities::app::App::fetch_all(token)?;
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
