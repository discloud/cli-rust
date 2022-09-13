use crate::entities::moderator::Feature;
use spinners::*;
fn subtract_vecs<T>(v1: &Vec<T>, v2: &Vec<T>) -> Vec<T> 
    where T: Eq + Clone
{
    v1.iter().filter(|&x| !v2.contains(x)).cloned().collect()
}
pub fn deny(id: u128, features: Vec<Feature>) {
    let token = super::expect_token();
    let app_id = crate::handle_result!(super::ask_for_app(token.clone(), "modify the mod's permissions"));
    let mut spinner = Spinner::new(Spinners::Toggle2, "Removing the permissions...".into());
    let moderator = crate::handle_result!(crate::entities::moderator::Mod::fetch_mod(token.clone(), id, app_id));
    match moderator {
        Some(mut moderator) => {
            crate::handle_result!(moderator.set_features(subtract_vecs(&moderator.get_features(), &features), token.clone()), spinner);
            spinner.stop_with_message(super::format_log(&format!("{:?} were removed successfully!", features)));
        },
        None => {
            spinner.stop_with_message(super::format_err("That moderator doesn't exist!"));
        }
    }
}