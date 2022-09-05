use directories::ProjectDirs;
/// Returns config base paths according to the conventions of the OS
pub fn get_proj_dir() -> Option<std::path::PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "Discloud",  "Discloud Cli") {
        Some(proj_dirs.config_dir().to_path_buf())
    } else {
        None
    }
}
/// Pushes file to the path returned by get_proj_dir()
pub fn get_path(file: &str) -> Option<std::path::PathBuf> {
    let mut result = get_proj_dir()?;
    result.push(file.to_string());
    Some(result)
}