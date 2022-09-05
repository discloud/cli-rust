use crate::api_url;

pub fn login(token: String) -> std::io::Result<()>{
    let token_file = crate::config_dir::get_path(".discloud_token").unwrap();
    std::fs::write(token_file, token)?;
    Ok(())
}
pub fn get_token() -> std::io::Result<String> {
    let token_file = crate::config_dir::get_path(".discloud_token").unwrap();
    std::fs::read_to_string(token_file)
}
pub fn validate_token() -> bool{
    match get_token() {
        Ok(token) => {
            let client = reqwest::blocking::Client::new();
            let req = client.get(concat!(api_url!(), "/user"))
                .header("api-token", token);
            if let Ok(res) = req.send() {
                res.status().is_success()
            } else {
                false
            }
        },
        Err(_) => {
            false
        }
    }
}