use spinners::*;
#[tracing::instrument]
pub fn restart(teams: bool) {
    let token = super::expect_token();
    match super::ask_for_app_id(token.clone(), "restart", teams) {
        Ok(app_id) => {
            let mut spinner = Spinner::new(Spinners::Earth, "Restarting your app".into());
            match crate::entities::app::App::restart(token, app_id, teams) {
                Ok(()) => {
                    spinner.stop_with_message(super::format_log("Your app is up!"));
                }
                Err(err) => {
                    super::err(&format!("Couldn't restart your app: {}", err));
                    std::process::exit(1);
                }
            }
        }
        Err(err) => {
            super::err(&format!("Couldn't fetch apps from api: {}", err));
            std::process::exit(1);
        }
    }
}
