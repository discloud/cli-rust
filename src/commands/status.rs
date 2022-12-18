use colored::Colorize;
#[tracing::instrument]
pub fn status(teams: bool) {
    let token = super::expect_token();
    match super::ask_for_app(token, "show the status", teams) {
        Ok(app) => {
            println!("Your app is {}", if app.online {
                "on".green()
            } else {
                "off".red()
            });
        }
        Err(err) => {
            super::err(&format!("Couldn't fetch apps from api: {}", err));
            std::process::exit(1);
        }
    }
}
