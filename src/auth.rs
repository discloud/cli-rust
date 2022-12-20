use crate::api_url;

#[tracing::instrument]
pub fn login(token: String) -> std::io::Result<()> {
    let token_file = crate::config_dir::get_path(".discloud_token").unwrap();
    std::fs::write(token_file, token)?;
    Ok(())
}
#[tracing::instrument]
pub fn get_token() -> std::io::Result<String> {
    if let Ok(token) = std::env::var("DISCLOUD_TOKEN") {
        Ok(token)
    } else {
        let token_file = crate::config_dir::get_path(".discloud_token").unwrap();
        std::fs::read_to_string(token_file)
    }
}
#[tracing::instrument]
pub fn validate_token() -> bool {
    match get_token() {
        Ok(token) => {
            let client = reqwest::blocking::Client::new();
            let req = client
                .get(concat!(api_url!(), "/user"))
                .header("api-token", token);
            if let Ok(res) = req.send() {
                res.status().is_success()
            } else {
                false
            }
        }
        Err(_) => false,
    }
}
