use std::fs;

pub struct FileData {
    pub name: String,
    pub path: Option<String>,
    pub content: String,
}

pub fn get_dir_opt(folder: &str) -> Option<std::path::PathBuf> {
    #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
    let dir_opt = {
        if let Ok(executable_dir) = std::env::current_exe() {
            let mut dir = executable_dir;
            dir.pop();
            for part in folder.split('/') {
                if part == "." {
                    continue;
                }
                dir.push(part);
            }
            Some(dir)
        } else {
            log::error!("Couldn't load the executable directory and therefore couldn't locate the {folder} folder");
            None
        }
    };
    #[cfg(target_os = "android")]
    let dir_opt: Option<std::path::PathBuf> = Some(folder.into());
    dir_opt
}

pub fn load_all_files_folder(folder: &str) -> Vec<FileData> {
    let mut files_data = Vec::new();
    let dir_opt = get_dir_opt(folder);
    if let Some(dir) = dir_opt {
        match dir.try_exists() {
            Ok(false) | Err(_) => {
                log::error!("The {dir:?} directory (constructed from the {folder} folder) was not found");
            }
            Ok(true) => {
                // The folder does exist
                let files = fs::read_dir(&dir);
                if let Ok(files) = files {
                    for file in files.flatten() {
                        let path = file.path();
                        let file_name = path.file_name();
                        if file_name.is_none() {
                            continue;
                        }
                        let file_name = file_name.unwrap().to_str();
                        if file_name.is_none() {
                            continue;
                        }
                        let file_name = file_name.unwrap().to_string();
                        let file_content = fs::read_to_string(&path);
                        if let Ok(file_content) = file_content {
                            files_data.push(FileData {
                                name: file_name,
                                path: path.to_str().map(|p| p.to_owned()),
                                content: file_content,
                            })
                        }
                    }
                }
            }
        }
    } else {
        log::error!("Failed to construct the directory (from the {folder} folder)");
    };
    files_data
}
