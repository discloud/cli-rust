pub mod auth;
pub mod config_dir;
pub mod entities;
mod commands;
use clap::*;
#[macro_export]
macro_rules! api_url {
    () => {"https://api.discloud.app/v2"};
    ($api:expr) => {format!("{}{}", $crate::api_url!(), $api)};
    ($api:literal) => {concat!($crate::api_url!(), $api)}
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
        )
        .subcommand(
            Command::new("init")
                .about("Creates a discloud.config file")
                .alias("i")
        )
        .subcommand(
            Command::new("upload")
                .about("Creates an app on discloud")
                .alias("up")
        )
        .subcommand(
            Command::new("commit")
                .about("Commits to an app on discloud. If you have more than one app, it will ask which app you want to commit to.")
                .alias("c")
        );
    let matches = cmd.get_matches();
    match matches.subcommand() {
        Some(("login", login_matches)) => commands::login::login(login_matches),
        Some(("authstatus", _)) => commands::authstatus::authstatus(),
        Some(("init", _)) => commands::init::init(),
        Some(("upload", _)) => {commands::upload::upload(); Ok(())},
        Some(("commit", _)) => {commands::commit::commit(); Ok(())},
        _ => unreachable!()
    }
}
