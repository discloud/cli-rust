use crate::{commands::{expect_token, ask_for_app}, entities::{moderator::Mod, app::App, user::fetch_user}};
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
pub fn remove(id: u128) {
    let token = expect_token();
    let app_id = handle_result!(ask_for_app(token.clone(), "remove the moderator"));
    let app = handle_result!(App::fetch(token.clone(), app_id));
    let user = handle_result!(fetch_user(token.clone()));
    let moderator = handle_result!(Mod::fetch_mod(token.clone(), &user, &app));
    if let Some(moderator) = moderator {
        handle_result!(moderator.remove(token.clone()));
        super::log(&format!("{} was removed from your app!", id));
    } else {
        super::err(&format!("{} isn't a moderator on {} ({})", id, app.name, app.id));
        std::process::exit(1);
    }
}