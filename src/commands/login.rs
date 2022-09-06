use clap::*;
pub fn login(matches: &ArgMatches) -> std::io::Result<()>{
    let token = matches.get_one::<String>("token").unwrap().clone();
    if let Err(err) = crate::auth::login(token) {
        super::err(format!("Couldn't save the token: {}", err.kind().to_string()).as_str());
        Err(err)
    } else {
        super::log("Token saved successfully");
        super::check_token();
        Ok(())
    }
}