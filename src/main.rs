pub mod auth;
pub mod config_dir;
use std::io::ErrorKind;
use clap::*;
use colored::Colorize;
#[macro_export]
macro_rules! api_url {
    () => {"https://api.discloud.app/v2"}
}
fn main() -> std::io::Result<()>{
    if let Some(dir) = config_dir::get_proj_dir() {
        std::fs::create_dir_all(dir)?;
    } else {
        eprintln!("ERROR: Couldn't find a directory for config files.");
        return Ok(());
    }
    let _guard = sentry::init(("https://dcfffa2a0b34450c980b4dff8c479a45@o1394903.ingest.sentry.io/6719901", sentry::ClientOptions {
        release: sentry::release_name!(),
        ..Default::default()
    }));
    let cmd = Command::new("discloud")
        .about("Blazingly Fast CLI for discloud")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Tiago Dinis")
        .subcommand(
            Command::new("login")
                .about("Sets the Discloud API token, use .api command on #commands to generate one")
                .alias("l")
                .arg(
                    Arg::new("token")
                        .required(true)
                        .action(ArgAction::Set)
                )
        )
        .subcommand(
            Command::new("authstatus")
                .about("Checks if you're logged in")
        );
    let matches = cmd.get_matches();
    match matches.subcommand() {
        Some(("login", login_matches)) => {
            if let Err(err) = auth::login(login_matches.get_one::<String>("token").unwrap().clone()) {
                eprintln!("{}{}{}", "Couldn't save the token: ".red(), err.kind().to_string().red(), "✘".red());
                return Err(err);
            } else {
                println!("{}", "Token saved successfully ✔".green());
                if !auth::validate_token() {
                    eprintln!("{} {}", "WARN:".yellow().bold(), "Your token is invalid!".yellow())
                }
            }
        },
        Some(("authstatus", _)) => {
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
        }
        _ => unreachable!()
    }
    Ok(())
}
