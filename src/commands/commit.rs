use spinners::Spinner;
use std::iter::Iterator;
use crate::zip_directory::commit as commit_zip;
use crate::zip_directory::{get_zip_file_path, zip_dir_to_file};
#[tracing::instrument]
pub fn commit(teams: bool) {
    let token = super::expect_token();
    let app_id = match super::ask_for_app_id(token.clone(), "commit", teams) {
        Ok(app_id) => app_id,
        Err(error) => {
            super::err(&format!("Couldn't fetch apps: {}", error));
            std::process::exit(1);
        }
    };

    let src_dir = ".";
    let dst_file = get_zip_file_path();
    match zip_dir_to_file(src_dir, dst_file.to_str().unwrap()) {
        Ok(_) => {}
        Err(e) => super::err(&format!("Failed to zip: {:?}", e)),
    }
    let mut spinner = Spinner::new(spinners::Spinners::Earth, "Committing app...".to_string());
    let msg = match commit_zip(token, app_id, teams) {
        Ok(()) => if !teams {super::format_log("Your app was updated successfully!")} else {super::format_log("Your buddy's app was updated!")},
        Err(err) => super::format_err(&err),
    };
    spinner.stop_with_message(msg);
}
