pub mod login;
pub mod authstatus;
pub mod init;
use spinners::*;
use colored::Colorize;
pub fn check_token() {
    let mut validate_spinner = Spinner::new(Spinners::Dots12, "Checking token".into());
    validate_spinner.stop_with_message(
        if crate::auth::validate_token() {
            format_log("Your token is valid!")
        } else {
            format_err("Your token is invalid!")
        }
    );
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