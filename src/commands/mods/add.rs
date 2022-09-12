use spinners::Spinner;

use crate::{commands::{expect_token, ask_for_app}, entities::{moderator::{Mod, Feature}}};

#[tracing::instrument]
pub fn add(id: u128) {
    let token = expect_token();
    let app_id = crate::handle_result!(ask_for_app(token.clone(), "add a moderator"));
    let mut spinner = Spinner::new(spinners::Spinners::Bounce, format!("Adding {} as a moderator", id));
    
    let moderator = crate::handle_result!(Mod::new(token.clone(), id, app_id, vec![Feature::SeeLogs, Feature::Status]), spinner);
    spinner.stop_with_message(super::format_log(&format!("Permissions {:?} have been given to {}", moderator.get_features(), id)));
}