pub mod auth;
mod commands;
pub mod config_dir;
pub mod entities;
pub mod zip_directory;
use clap::*;
use entities::moderator::Feature;
use tracing_subscriber::prelude::*;
#[macro_export]
macro_rules! api_url {
    () => {
        "https://api.discloud.app/v2"
    };
    ($api:expr) => {
        format!("{}{}", $crate::api_url!(), $api)
    };
    ($api:literal) => {
        concat!($crate::api_url!(), $api)
    };
}
fn main() {
    tracing_subscriber::Registry::default()
        .with(sentry::integrations::tracing::layer())
        .init();
    fn parse_feature(feature: &str) -> Result<Feature, String> {
        match feature {
            "start" => Ok(Feature::Start),
            "stop" => Ok(Feature::Stop),
            "restart" => Ok(Feature::Restart),
            "logs" => Ok(Feature::SeeLogs),
            "commit" => Ok(Feature::Commit),
            "status" => Ok(Feature::Status),
            "setram" => Ok(Feature::SetRam),
            "backup" => Ok(Feature::Backup),
            _ => Err(format!("Invalid permission: {}", feature)),
        }
    }
    let _guard = sentry::init((
        "https://0512a7bb28624cfc848cdad08f2186a7@sentry.discloudbot.com/3",
        sentry::ClientOptions {
            release: sentry::release_name!(),
            traces_sample_rate: if cfg!(debug_assertions) { 1.0 } else { 0.2 },
            ..Default::default()
        },
    ));
    if let Some(dir) = config_dir::get_proj_dir() {
        if let Err(err) = std::fs::create_dir_all(dir) {
            eprintln!("ERROR: Couldn't create a directory for config files: {err}");
        }
    } else {
        eprintln!("ERROR: Couldn't find a directory for config files.");
        return;
    }
    let cmd = Command::new("discloud")
        .about("Blazingly Fast CLI for discloud")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Tiago Dinis")
        .version(env!("CARGO_PKG_VERSION"))
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
        )
        .subcommand(
            Command::new("remove")
                .about("Removes an app. If you have more than one app, it will ask which app you want to delete.")
                .alias("rm")
                .alias("rb")
        )
        .subcommand(
            Command::new("apps")
                .about("Shows all your apps.")
                .alias("ls")
                .alias("list")
        )
        .subcommand(
            Command::new("stop")
                .about("Stops an app.")
                .alias("shutdown")
                .alias("down")
        )
        .subcommand(
            Command::new("start")
                .about("Starts a stopped app.")
        )
        .subcommand(
            Command::new("restart")
                .about("Restarts an app.")
                .alias("reboot")
                .alias("r")
        )
        .subcommand(
            Command::new("logs")
                .about("Prints logs of an app.")
                .alias("terminal")
                .alias("t")
        )
        .subcommand(
            Command::new("aboutme")
                .about("Shows information about you.")
                .alias("user")
        )
        .subcommand(
            Command::new("teams")
            .about("Allows to interact with other people's bots, you can use this once someone adds you as a moderator.")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(
                Command::new("apps")
                    .about("Shows all bots you have access to.")
                    .alias("ls")
                    .alias("list")
            )
            .subcommand(
                Command::new("start")
                    .about("Starts a stopped app.")
            )
            .subcommand(
                Command::new("restart")
                    .about("Restarts an app.")
                    .alias("reboot")
                    .alias("r")
            )
            .subcommand(
                Command::new("logs")
                    .about("Prints logs of an app.")
                    .alias("terminal")
                    .alias("t")
            )
                
            .subcommand(
                Command::new("stop")
                    .about("Stops an app.")
                    .alias("shutdown")
                    .alias("down")
            )
            
            .subcommand(
                Command::new("commit")
                    .about("Commits to an app on discloud. If you have more than one shared app, it will ask which app you want to commit to.")
                    .alias("c")
            )
        )
        .subcommand(
            Command::new("mods")
                .about("Manages your apps' mods")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .alias("m")
                .subcommand(
                    Command::new("add")
                        .about("Adds a mod to an app, by default, the mod can only see the logs and status, use `discloud mods allow` to allow more actions.")
                        .arg(Arg::new("id").value_parser(value_parser!(u128)).action(clap::ArgAction::Set).required(true))
                )
                .subcommand(
                    Command::new("remove")
                        .alias("rm")
                        .about("Removes a moderator from your app.")
                        .arg(Arg::new("id").value_parser(value_parser!(u128)).action(clap::ArgAction::Set).required(true))
                )
                .subcommand(
                    Command::new("allow")
                        .about("Gives permissions to a moderator")
                        .arg(Arg::new("id").value_parser(value_parser!(u128)).action(clap::ArgAction::Set)
                                .required(true))
                        .arg(
                            Arg::new("perm")
                                .value_parser(parse_feature)
                                .action(clap::ArgAction::Append)
                                .required(true)
                        )
                )
                .subcommand(
                    Command::new("deny")
                        .about("Removes permissions from a moderator")
                        .arg(Arg::new("id").value_parser(value_parser!(u128)).action(clap::ArgAction::Set)
                                .required(true))
                        .arg(
                            Arg::new("perm")
                                .value_parser(parse_feature)
                                .action(clap::ArgAction::Append)
                                .required(true)
                        )
                )
                .after_help("Be careful with what people you add and what permissions you give: With Great Power comes Great Responsability.")
        );
    let matches = cmd.get_matches();
    match matches.subcommand() {
        Some(("login", login_matches)) => {
            if let Err(err) = commands::login::login(login_matches) {
                sentry::capture_error(&err);
            }
            
        },
        Some(("authstatus", _)) => {
            if let Err(err) = commands::authstatus::authstatus() {
                sentry::capture_error(&err);
            }
            
        },
        Some(("init", _)) => {
            if let Err(err) = commands::init::init() {
                sentry::capture_error(&err);
            }
            
        },
        Some(("upload", _)) => {
            commands::upload::upload();
            
        }
        Some(("commit", _)) => {
            commands::commit::commit(false);
            
        }
        Some(("remove", _)) => {
            commands::remove::remove();
            
        }
        Some(("apps", _)) => {
            commands::apps::apps(false);
            
        }
        Some(("stop", _)) => {
            commands::stop::stop(false);
            
        }

        Some(("start", _)) => {
            commands::start::start(false);
            
        }
        Some(("restart", _)) => {
            commands::restart::restart(false);
            
        }
        Some(("logs", _)) => {
            commands::logs::logs(false);
            
        }
        Some(("aboutme", _)) => {
            commands::aboutme::aboutme();
            
        }
        Some(("mods", matches)) => match matches.subcommand() {
            Some(("add", matches)) => {
                let id: u128 = *matches.get_one("id").unwrap();
                commands::mods::add::add(id);
                
            }
            Some(("remove", matches)) => {
                let id: u128 = *matches.get_one("id").unwrap();
                commands::mods::remove::remove(id);
                
            }
            Some(("deny", matches)) => {
                let id: u128 = *matches.get_one("id").unwrap();
                let features: Vec<Feature> = matches.get_many("perm").unwrap().cloned().collect();
                commands::mods::deny::deny(id, features);
                
            }
            Some(("allow", matches)) => {
                let id: u128 = *matches.get_one("id").unwrap();
                let features: Vec<Feature> = matches.get_many("perm").unwrap().cloned().collect();
                commands::mods::allow::allow(id, features);
                
            }
            cmd => {
                eprintln!("{:#?} command is not implemented yet.", cmd.unwrap().0);
                
            }
        },
        Some(("teams", matches)) => match matches.subcommand() {
            Some(("commit", _)) => {
                commands::commit::commit(true);
                
            }
            Some(("apps", _)) => {
                commands::apps::apps(true);
                
            }

            Some(("stop", _)) => {
                commands::stop::stop(true);
                
            }
                
            Some(("start", _)) => {
                commands::start::start(true);
                
            }
            Some(("restart", _)) => {
                commands::restart::restart(true);
                
            }
            Some(("logs", _)) => {
                commands::logs::logs(true);
                
            }
            cmd => {
                eprintln!("{:#?} command is not implemented yet.", cmd.unwrap().0);
                
            }
        },
        cmd => {
            eprintln!("{:#?} command is not implemented yet.", cmd.unwrap().0);
            
        }
    }
}
