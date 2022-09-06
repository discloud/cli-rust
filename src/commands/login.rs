use clap::*;
pub const LOGIN_COMMAND: Command = Command::new("login")
    .about("Sets the Discloud API token, use .api command on #commands to generate one")
    .alias("l")
    .arg(
        Arg::new("token")
            .required(true)
            .action(ArgAction::Set)
    );
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