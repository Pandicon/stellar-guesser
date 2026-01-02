fn get_exe_path() -> Result<std::path::PathBuf, std::io::Error> {
    let exe_path = std::env::current_exe()?;

    let exe_dir = exe_path.parent().unwrap_or(&exe_path).to_path_buf();
    Ok(exe_dir)
}

pub fn read_file_relative(file_path: impl Into<std::path::PathBuf>) -> Result<crate::files_handling::FileInfo, std::io::Error> {
    let exe_dir = get_exe_path()?;
    super::read_file_relative_filesystem(exe_dir, file_path)
}

pub fn read_dir_relative(dir_path: impl Into<std::path::PathBuf>) -> Result<Vec<crate::files_handling::FileInfo>, std::io::Error> {
    let exe_dir = get_exe_path()?;
    super::read_dir_relative_filesystem(exe_dir, dir_path)
}

pub fn get_path_relative(relative_path: impl Into<std::path::PathBuf>) -> Result<std::path::PathBuf, std::io::Error> {
    let base_path = get_exe_path()?;
    Ok(super::get_path_relative_inner(base_path, relative_path))
}

pub fn save_file_at_path(contents: String, save_path: impl Into<std::path::PathBuf>) -> Result<(), std::io::Error> {
    let save_path = save_path.into();
    if let Some(dir) = save_path.parent() {
        if !dir.exists() {
            if let Err(err) = std::fs::create_dir_all(dir) {
                log::error!("Failed to create the folders for the file: {err}");
            }
        }
    } else {
        log::warn!("No file folder: {:?}", save_path);
    }
    std::fs::write(save_path, contents)
}

pub fn save_file_by_user(base_path: impl Into<std::path::PathBuf>, _default_file_name: &str, contents: String, filter_name: &str, extensions: &[&str]) -> Option<Result<(), std::io::Error>> {
    let save_path_opt: Option<std::path::PathBuf> = {
        let dialog = rfd::FileDialog::new().add_filter(filter_name, extensions).set_directory(base_path.into());
        dialog.save_file()
    };
    save_path_opt.map(|save_path| save_file_at_path(contents, save_path))
}

pub fn save_file_by_user_relative_base_path(
    relative_base_path: impl Into<std::path::PathBuf>,
    default_file_name: &str,
    contents: String,
    filter_name: &str,
    extensions: &[&str],
) -> Result<Option<Result<(), std::io::Error>>, std::io::Error> {
    crate::files_handling::get_path_relative(relative_base_path).map(|base_path| save_file_by_user(base_path, default_file_name, contents, filter_name, extensions))
}
