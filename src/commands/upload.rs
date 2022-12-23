use spinners::Spinner;
use crate::zip_directory::{upload_zip, zip_dir_to_file, get_zip_file_path};

#[tracing::instrument]
pub fn upload() {
    let token = super::expect_token();
    let src_dir = ".";
    let dst_file = get_zip_file_path();
    match zip_dir_to_file(src_dir, dst_file.to_str().unwrap()) {
        Ok(_) => super::log("Your project is ready to upload!"),
        Err(e) => super::err(&format!("Failed to zip: {:?}", e)),
    }
    let mut spinner = Spinner::new(spinners::Spinners::Earth, "Uploading app...".to_string());
    let msg = match upload_zip(token) {
        Ok(()) => super::format_confetti("Your app was successfully uploaded!"),
        Err(err) => super::format_err(&err),
    };
    spinner.stop_with_message(msg);
}
