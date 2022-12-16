use spinners::*;
#[tracing::instrument]
pub fn start() {
    let token = super::expect_token();
    match super::ask_for_app(token.clone(), "start") {
        Ok(app_id) => {
            let mut spinner = Spinner::new(Spinners::Earth, "Starting your app".into());
            match crate::entities::app::App::start(token, app_id) {
                Ok(()) => {
                    spinner.stop_with_message(super::format_log("Your app is up!"));
                }
                Err(err) => {
                    super::err(&format!("Couldn't start your app: {}", err));
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
