use colored::Colorize;
use spinners::Spinner;
use std::io::prelude::*;
use std::io::{Seek, Write};
use std::iter::Iterator;
use zip::result::ZipError;
use zip::write::FileOptions;

use std::fs::File;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};
#[tracing::instrument]
fn get_zip_file_path() -> PathBuf {
    let mut dst_file = std::env::temp_dir();
    dst_file.push("discloud.zip");
    dst_file
}
#[tracing::instrument]
pub fn commit() {
    let token = super::expect_token();
    let app_id = match super::ask_for_app(token.clone(), "commit") {
        Ok(app_id) => app_id,
        Err(error) => {
            super::err(&format!("Couldn't fetch apps: {}", error));
            std::process::exit(1);
        }
    };

    let src_dir = ".";
    let dst_file = get_zip_file_path();
    match zip_dir_to_file(src_dir, dst_file.to_str().unwrap(), METHOD_DEFLATED) {
        Ok(_) => {}
        Err(e) => super::err(&format!("Failed to zip: {:?}", e)),
    }
    let mut spinner = Spinner::new(spinners::Spinners::Earth, "Committing app...".to_string());
    let msg = match upload_zip(token, app_id) {
        Ok(()) => super::format_log("Your app was successfully commited!"),
        Err(err) => super::format_err(&err),
    };
    spinner.stop_with_message(msg);
}

const METHOD_DEFLATED: zip::CompressionMethod = zip::CompressionMethod::Deflated;

fn zip_dir<T>(
    it: &mut dyn Iterator<Item = DirEntry>,
    prefix: &str,
    writer: T,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()>
where
    T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();
        if path.is_file() {
            print!("⌛ Zipping file: {}\r", name.to_str().unwrap());
            zip.start_file(name.to_str().unwrap(), options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
            println!("{}", "✔".green().bold());
        } else if !name.as_os_str().is_empty() {
            zip.add_directory(name.to_str().unwrap(), options)?;
        }
    }
    zip.finish()?;
    Result::Ok(())
}

#[tracing::instrument]
fn zip_dir_to_file(
    src_dir: &str,
    dst_file: &str,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()> {
    if !Path::new(src_dir).is_dir() {
        return Err(ZipError::FileNotFound);
    }
    let writer = File::create(dst_file).unwrap();

    let walkdir = WalkDir::new(src_dir);
    let it = walkdir.into_iter();

    zip_dir(
        &mut it.filter_map(|e| {
            if let Ok(e) = e {
                let components = e.path().components().collect::<Vec<_>>();
                if components.len() < 2 {
                    Some(e)
                } else {
                    match components[1].as_os_str().to_str().unwrap() {
                        "target" | ".git" | "build" | "out" | "node_modules" | ".gitignore" => None,
                        _ => Some(e),
                    }
                }
            } else {
                None
            }
        }),
        src_dir,
        writer,
        method,
    )?;

    Ok(())
}
#[tracing::instrument]
fn upload_zip(token: String, app_id: u128) -> Result<(), String> {
    let file_path = get_zip_file_path();
    let file_path = file_path.to_str().unwrap();
    let client = reqwest::blocking::Client::builder()
        .timeout(None)
        .build()
        .unwrap();
    let form = reqwest::blocking::multipart::Form::new().file("file", file_path);
    match form {
        Err(err) => Err(format!("Couldn't open zip file: {}", err)),
        Ok(form) => {
            let req = client
                .put(crate::api_url!(format!("/app/{}/commit", app_id)))
                .multipart(form)
                .header("api-token", token);
            let res = req.send();
            match res {
                Err(err) => Err(err.to_string()),
                Ok(res) => {
                    if res.status().is_success() {
                        Ok(())
                    } else {
                        Err(format!(
                            "Commit failed: API returned {} http code: {}",
                            res.status().as_u16(),
                            res.text().unwrap()
                        ))
                    }
                }
            }
        }
    }
}
