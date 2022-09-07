use colored::Colorize;
pub fn apps() {
    let token = super::expect_token();
    match crate::entities::app::App::fetch_all(token.clone()) {
        Ok(apps) => {
            println!("Your apps:");
            for app in apps {
                println!("- {}: ({}) {}", app.name.green(), app.lang.yellow(), app.id.to_string().bright_black());
            }
        }
        Err(err) => {
            super::err(&err.to_string());
        }
    }
}