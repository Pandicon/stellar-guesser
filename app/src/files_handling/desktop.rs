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
