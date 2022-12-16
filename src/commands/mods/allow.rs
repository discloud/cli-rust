use crate::entities::moderator::Feature;
use spinners::*;
pub fn allow(id: u128, features: Vec<Feature>) {
    let token = super::expect_token();
    let app_id = crate::handle_result!(super::ask_for_app(
        token.clone(),
        "modify the mod's permissions"
    ));
    let mut spinner = Spinner::new(Spinners::Pong, "Adding the permissions...".into());
    let moderator = crate::handle_result!(crate::entities::moderator::Mod::fetch_mod(
        token.clone(),
        id,
        app_id
    ));
    match moderator {
        Some(mut moderator) => {
            let mut feats = moderator.get_features();
            feats.append(&mut features.clone());
            crate::handle_result!(moderator.set_features(feats, token.clone()), spinner);
            spinner.stop_with_message(super::format_log(&format!(
                "{:?} were added successfully!",
                features
            )));
        }
        None => {
            spinner.stop_with_message(super::format_err("That moderator doesn't exist!"));
        }
    }
}
