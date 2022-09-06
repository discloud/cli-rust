use colored::Colorize;
use dialoguer::theme::ColorfulTheme;

fn vec_from_str(s: String) -> Vec<String> {
    s.split(",").map(|s|s.trim().into()).collect()
}

#[derive(Default)]
enum AppTyp {
    #[default]
    Bot,
    Site,
}
#[derive(Default)]
struct App {
    typ: AppTyp,
    id: u128,
    subdomain: String,
    ram: u64,
    main: String,
    autorestart: bool,
    apt: Vec<String>,
}
impl App {
    fn get_config(&self) -> String {
        match &self.typ {
            AppTyp::Site => {
                if self.apt.len() > 0 {
                    format!("ID={}\nMAIN={}\nAUTORESTART={}\nRAM={}\nAPT={}\nTYPE=site\nVERSION=latest", self.subdomain, self.main, self.autorestart, self.ram, self.apt.join(","))
                } else {
                    format!("ID={}\nMAIN={}\nAUTORESTART={}\nRAM={}\nTYPE=site\nVERSION=latest", self.subdomain, self.main, self.autorestart, self.ram)
                }
            }
            AppTyp::Bot => {
                if self.apt.len() > 0 {
                    format!("ID={}\nMAIN={}\nAUTORESTART={}\nRAM={}\nAPT={}\nTYPE=bot\nVERSION=latest", self.id, self.main, self.autorestart, self.ram, self.apt.join(","))
                } else {
                    format!("ID={}\nMAIN={}\nAUTORESTART={}\nRAM={}\nTYPE=bot\nVERSION=latest", self.id, self.main, self.autorestart, self.ram)
                }
            }
        }
    }
}
pub fn init() -> std::io::Result<()> {
    use dialoguer::Input;
    if std::path::Path::new("discloud.config").exists() {
        super::warn("discloud.config already exists");
    }
    let typ: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Type")
        .default("bot".into())
        .show_default(true)
        .validate_with(|t: &String| {
            if t == "bot" || t == "site" {
                Ok(())
            } else {
                Err("Only `bot` and `site` are valid")
            }
        })
        .interact_text()?;
    let mut app: App = Default::default();
    match typ.as_str() {
        "bot" => {
            app.typ = AppTyp::Bot;
            app.id = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Bot ID")
                .interact_text()?;
        }
        "site" => {
            app.typ = AppTyp::Site;
            app.subdomain = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Subdomain")
                .interact_text()?;
        }
        _ => unreachable!(),
    }
    app.main = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Main File")
        .interact_text()?;
    app.autorestart = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("AutoRestart?")
        .default(false)
        .interact_text()?;
    app.ram = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Memory (MB)")
        .interact_text()?;
    let apt: String = Input::with_theme(&ColorfulTheme::default())
    .with_prompt("APT Packages")
    .allow_empty(true)
    .interact_text()?;
    if apt.len() > 0 {
        app.apt = vec_from_str(apt);
    }
    std::fs::write("discloud.config", app.get_config())?;
    super::log("discloud.config was created succesfully!");
    Ok(())
}
