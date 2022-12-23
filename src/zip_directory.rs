use ignore::{DirEntry, WalkBuilder};
use serde::Deserialize;
use std::io::prelude::*;
use std::io::{Seek, Write};
use std::iter::Iterator;
use zip::result::ZipError;
use zip::write::FileOptions;

use std::fs::File;
use std::path::{Path, PathBuf};
pub fn get_zip_file_path() -> PathBuf {
    let mut dst_file = std::env::temp_dir();
    dst_file.push("discloud.zip");
    dst_file
}

const METHOD_DEFLATED: zip::CompressionMethod = zip::CompressionMethod::Deflated;

pub fn zip_dir<T>(
    it: &mut dyn Iterator<Item = Result<DirEntry, ignore::Error>>,
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
    for entry in it.flatten() {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();
        if path.is_file() {
            print!("⌛ Zipping file: {}\r", name.to_str().unwrap());
            zip.start_file(name.to_str().unwrap(), options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            zip.add_directory(name.to_str().unwrap(), options)?;
        }
    }
    zip.finish()?;
    Result::Ok(())
}

pub fn zip_dir_to_file(src_dir: &str, dst_file: &str) -> zip::result::ZipResult<()> {
    if !Path::new(src_dir).is_dir() {
        return Err(ZipError::FileNotFound);
    }
    let writer = File::create(dst_file).unwrap();
    let mut walkdir = WalkBuilder::new(src_dir)
        .ignore(true)
        .add_custom_ignore_filename(".discloudignore")
        .build();

    zip_dir(&mut walkdir, src_dir, writer, METHOD_DEFLATED)?;

    Ok(())
}
pub fn upload_zip(token: String) -> Result<(), String> {
    #[derive(Deserialize)]
    struct UploadResponse {
        status: String,
        message: Option<String>,
        logs: Option<String>,
    }
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
                .post(crate::api_url!("/upload"))
                .multipart(form)
                .header("api-token", token);
            let res = req.send();
            match res {
                Err(err) => Err(err.to_string()),
                Ok(res) => {
                    let status = res.status();
                    let res: UploadResponse = res.json().unwrap();
                    if res.status == "error" {
                        if let Some(logs) = res.logs {
                            Err(format!(
                                "Upload failed: API Returned {}: {}\nLogs:\n{}",
                                status.as_u16(),
                                res.message.unwrap(),
                                logs
                            ))
                        } else {
                            Err(format!(
                                "Upload failed: API Returned {}: {}",
                                status.as_u16(),
                                res.message.unwrap()
                            ))
                        }
                    } else {
                        Ok(())
                    }
                }
            }
        }
    }
}

pub fn commit(token: String, app_id: String, teams: bool) -> Result<(), String> {
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
                .put(crate::api_url!(format!(
                    "/{}/{}/commit",
                    if teams { "team" } else { "app" },
                    app_id
                )))
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
