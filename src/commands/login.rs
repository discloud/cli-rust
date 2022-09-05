use clap::*;
use colored::Colorize;
pub fn login(matches: &ArgMatches) -> std::io::Result<()>{
    let token = matches.get_one::<String>("token").unwrap().clone();
    if let Err(err) = crate::auth::login(token) {
        eprintln!("{} Couldn't save the token: {}", "✘".red(), err.kind().to_string());
        Err(err)
    } else {
        println!("{} Token saved successfully", "✔".green());
        super::check_token();
        Ok(())
    }
}