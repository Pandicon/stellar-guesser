#[cfg(target_os = "android")]
mod android;
#[cfg(target_os = "android")]
pub use android::*;
#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
mod desktop;
#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
pub use desktop::*;

use std::path::PathBuf;

pub struct FileInfo {
    name: Option<String>,
    path: std::path::PathBuf,
    contents: Vec<u8>,
}

impl FileInfo {
    pub fn new(name: Option<String>, path: std::path::PathBuf, contents: Vec<u8>) -> Self {
        Self { name, path, contents }
    }

    pub fn get_name(&self) -> &Option<String> {
        &self.name
    }

    pub fn get_path(&self) -> &std::path::PathBuf {
        &self.path
    }

    pub fn get_contents(&self) -> &[u8] {
        &self.contents
    }

    pub fn contents_as_string(&self) -> Result<String, std::io::Error> {
        use std::io::Read;

        let mut file_contents = String::new();
        match self.get_contents().read_to_string(&mut file_contents) {
            Ok(_) => Ok(file_contents),
            Err(e) => Err(e),
        }
    }

    pub fn contents_as_string_or_empty(&self) -> String {
        match self.contents_as_string() {
            Ok(s) => s,
            Err(e) => {
                log::error!("Failed to convert contents of file {:?} ({:?}) to string: {e}", self.path, self.get_contents());
                String::new()
            }
        }
    }
}

pub fn read_file_raw(file_path: impl Into<PathBuf>) -> Result<FileInfo, std::io::Error> {
    let file_path = file_path.into();
    let name = file_path.file_name().and_then(|n| n.to_str()).map(|c| c.to_owned());
    let contents = std::fs::read(&file_path)?;
    Ok(FileInfo { name, path: file_path, contents })
}

pub fn read_dir_raw(directory: impl Into<PathBuf>) -> Vec<FileInfo> {
    let directory = directory.into();
    match std::fs::read_dir(&directory) {
        Ok(files) => files
            .flatten()
            .filter_map(|f| {
                let path = f.path();
                match read_file_raw(&path) {
                    Ok(i) => Some(i),
                    Err(err) => {
                        log::error!("Failed to read file {:?}: {err}", path.to_str());
                        None
                    }
                }
            })
            .collect(),
        Err(err) => {
            log::error!("Failed to read directory {:?}: {err}", directory.to_str());
            Vec::new()
        }
    }
}

fn get_path_relative_inner(base_path: impl Into<PathBuf>, relative_path: impl Into<PathBuf>) -> std::path::PathBuf {
    let base_path = base_path.into();
    let base_path = base_path.canonicalize().unwrap_or(base_path);

    let full_path = base_path.join(relative_path.into());
    full_path.canonicalize().unwrap_or(full_path)
}

fn read_file_relative_filesystem(base_path: impl Into<PathBuf>, relative_path: impl Into<PathBuf>) -> Result<FileInfo, std::io::Error> {
    let base_path = base_path.into().canonicalize()?;

    let full_path = get_path_relative_inner(&base_path, relative_path);
    let canonical_full = full_path.canonicalize()?;
    log::debug!("Path traversal attempt check debug (file): (canonical path: {canonical_full:?}, base path: {base_path:?})");
    if !canonical_full.starts_with(&base_path) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            format!("Path traversal attempt detected (canonical path: {canonical_full:?}, base path: {base_path:?})"),
        ));
    }

    read_file_raw(canonical_full)
}

fn read_dir_relative_filesystem(base_path: impl Into<PathBuf>, relative_path: impl Into<PathBuf>) -> Result<Vec<FileInfo>, std::io::Error> {
    let base_path = base_path.into().canonicalize()?;

    let full_path = base_path.join(relative_path.into());
    let canonical_full = full_path.canonicalize()?;
    log::debug!("Path traversal attempt check debug (dir): (canonical path: {canonical_full:?}, base path: {base_path:?})");
    if !canonical_full.starts_with(&base_path) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            format!("Path traversal attempt detected (canonical path: {canonical_full:?}, base path: {base_path:?})"),
        ));
    }

    Ok(read_dir_raw(canonical_full))
}
