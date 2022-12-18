use spinners::Spinner;

use crate::handle_result;
use crate::{
    commands::{ask_for_app_id, expect_token},
    entities::{app::App, moderator::Mod},
};
#[tracing::instrument]
pub fn remove(id: u128) {
    let token = expect_token();
    let app_id = handle_result!(ask_for_app_id(token.clone(), "remove the moderator", false));
    let mut spinner = Spinner::new(
        spinners::Spinners::Moon,
        format!("Sending {} to the moon", id),
    );
    let app = handle_result!(App::fetch(token.clone(), app_id), spinner);
    let moderator = handle_result!(Mod::fetch_mod(token.clone(), id, app_id), spinner);
    if let Some(moderator) = moderator {
        handle_result!(moderator.remove(token), spinner);
        spinner.stop_with_message(super::format_log(&format!(
            "{} was removed from your app!",
            id
        )));
    } else {
        spinner.stop_with_message(super::format_err(&format!(
            "{} isn't a moderator on {} ({})",
            id, app.name, app.id
        )));
        std::process::exit(1);
    }
}
