pub mod auth;
pub mod config_dir;
mod commands;
use clap::*;
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
        Some(("login", login_matches)) => commands::login::login(login_matches),
        Some(("authstatus", _)) => commands::authstatus::authstatus(),
        _ => unreachable!()
    }
}
