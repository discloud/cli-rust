pub mod login;
pub mod authstatus;
pub mod init;
use spinners::*;
use colored::Colorize;
pub fn check_token() {
    let mut validate_spinner = Spinner::new(Spinners::Dots12, "Checking token".into());
    validate_spinner.stop_with_message(
        if crate::auth::validate_token() {
            format!("{} {}", "✔".green().bold(), "Your token is valid!")
        } else {
            format!("{} {}", "!".yellow().bold(), "Your token is invalid!")
        }
    );
}