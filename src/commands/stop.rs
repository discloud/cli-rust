use spinners::*;
#[tracing::instrument]
pub fn stop() {
    let token = super::expect_token();
    match super::ask_for_app(token.clone(), "shutdown") {
        Ok(app_id) => {
            let mut spinner = Spinner::new(Spinners::Pong, "Shutting down your app".into());
            match crate::entities::app::App::stop(token, app_id) {
                Ok(()) => {
                    spinner.stop_with_message(super::format_log("Your app is down!"));
                }
                Err(err) => {
                    super::err(&format!("Couldn't shutdown your app: {}", err));
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
