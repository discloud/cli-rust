use std::io::ErrorKind;
use colored::Colorize;
use crate::auth;
pub fn authstatus() -> std::io::Result<()> {
    match auth::get_token() {
        Ok(token) => {
            println!("You're already logged in!\n");
            let mut stars = String::new();
            for _ in 0..token.len()-5 {
                stars.push('*');
            }
            println!("{} Token: {}{}", "✔".green(), &token[..5], stars);
            if !auth::validate_token() {
                eprintln!("{} {}", "WARN:".yellow().bold(), "Your token is invalid!".yellow())
            }
        }
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                println!("{} You're not logged in yet!", "✘".red());
            },
            err => {
                eprintln!("{} Couldn't open token file: {}", "✘".red(), err);
            }
        } 
    }
    Ok(())
}