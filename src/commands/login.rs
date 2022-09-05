use clap::*;
use colored::Colorize;
use crate::auth;
pub fn login(matches: &ArgMatches) -> std::io::Result<()>{
    let token = matches.get_one::<String>("token").unwrap().clone();
    if let Err(err) = crate::auth::login(token) {
        eprintln!("{}{}{}", "Couldn't save the token: ".red(), err.kind().to_string().red(), "✘".red());
        Err(err)
    } else {
        println!("{}", "Token saved successfully ✔".green());
        if !auth::validate_token() {
            eprintln!("{} {}", "WARN:".yellow().bold(), "Your token is invalid!".yellow())
        }
        Ok(())
    }
}