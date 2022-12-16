use directories::ProjectDirs;
/// Returns config base paths according to the conventions of the OS
#[tracing::instrument]
pub fn get_proj_dir() -> Option<std::path::PathBuf> {
    ProjectDirs::from("com", "Discloud", "Discloud Cli").map(|p| p.config_dir().to_path_buf())
}
#[tracing::instrument]
/// Pushes file to the path returned by get_proj_dir()
pub fn get_path(file: &str) -> Option<std::path::PathBuf> {
    let mut result = get_proj_dir()?;
    result.push(file);
    Some(result)
}
