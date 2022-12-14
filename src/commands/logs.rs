use spinners::{Spinner, Spinners};

#[tracing::instrument]
pub fn logs(teams: bool) {
    let token = super::expect_token();
    match super::ask_for_app_id(token.clone(), "show the logs", teams) {
        Ok(app_id) => {
            let mut spinner = Spinner::new(Spinners::Bounce, "Downloading the logs".into());
            match crate::entities::app::App::get_logs(token, app_id, teams) {
                Ok(logs) => {
                    spinner.stop_with_message(logs);
                }
                Err(err) => {
                    super::err(&format!("Couldn't delete your app: {}", err));
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
