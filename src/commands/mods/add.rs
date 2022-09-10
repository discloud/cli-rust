use crate::{commands::{expect_token, ask_for_app}, entities::{moderator::{Mod, Feature}, app::App, user::fetch_user}};
macro_rules! handle_result {
    ($v:expr) => {
        match $v {
            Ok(v) => v,
            Err(err) => {
                super::err(&err.to_string());
                std::process::exit(1);
            }
        }
    }
}
pub fn add(id: u128) {
    let token = expect_token();
    let app_id = handle_result!(ask_for_app(token.clone(), "add a moderator"));
    let app = handle_result!(App::fetch(token.clone(), app_id));
    let user = handle_result!(fetch_user(token.clone()));
    let moderator = handle_result!(Mod::new(token.clone(), &user, &app, vec![Feature::SeeLogs, Feature::Status]));
    super::log(&format!("Permissions {:?} have been given to {}", moderator.get_features(), id));
}