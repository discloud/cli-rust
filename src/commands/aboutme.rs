use chrono::{Datelike, Timelike};
use colored::Colorize;

#[tracing::instrument]
pub fn aboutme() {
    let token = super::expect_token();
    match crate::entities::user::fetch_user(token) {
        Ok(user) => {
            println!("ID: {}", user.user_id.bright_black());
            println!("Plan: {}", color_plan(user.plan));
            if let Some(end_date) = user.plan_data_end {
                println!(
                    "Your plan ends at: {}/{}/{} {}:{}",
                    end_date.day(),
                    end_date.month(),
                    end_date.year(),
                    end_date.hour(),
                    end_date.minute()
                );   
            }
            if let Some(time_left) = user.last_data_left{
                println!(
                    "  Which means you have {} days left!",
                    time_left.days.to_string().green().bold()
                );
            }
            println!("Memory:");
            println!(
                "  Total: {}{}",
                user.total_ram_mb.to_string().green().bold(),
                "MB".green().bold()
            );
            println!(
                "  Used: {}{}",
                user.ram_used_mb.to_string().green().bold(),
                "MB".green().bold()
            );
            println!(
                "  Available: {}{}",
                (user.total_ram_mb - user.ram_used_mb)
                    .to_string()
                    .green()
                    .bold(),
                "MB".green().bold()
            );
            println!("Locale: {}", user.locale.blue());
        }
        Err(err) => super::err(&err.to_string()),
    }
}
#[tracing::instrument]
fn color_plan(plan: String) -> String {
    match plan.as_str() {
        "Free" => plan.bright_black().to_string(),
        "Carbon" => plan.bright_black().bold().to_string(),
        "Gold" => plan.yellow().bold().to_string(),
        "Platinum" => plan.blue().bold().to_string(),
        "Diamond" => plan.cyan().bold().to_string(),
        "Ruby" => plan.red().bold().to_string(),
        "Safira" | "Sapphire" => plan.bright_red().bold().to_string(),
        "Krypton" => plan.bright_green().bold().to_string(),
        "Special" => plan.bright_cyan().bold().to_string(),
        _ => plan,
    }
}
