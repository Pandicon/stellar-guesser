use std::fs;

pub struct FileData {
    pub name: String,
    pub content: String,
}

pub fn load_all_files_folder(folder: &str) -> Vec<FileData> {
    let mut files_data = Vec::new();
    #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
    let dir_opt = {
        if let Ok(executable_dir) = std::env::current_exe() {
            let mut images_addon_dir = executable_dir;
            images_addon_dir.pop();
            for part in folder.split('/') {
                if part == "." {
                    continue;
                }
                images_addon_dir.push(part);
            }
            Some(images_addon_dir)
        } else {
            log::error!("Couldn't load the executable directory and therefore couldn't locate the {folder} folder");
            None
        }
    };
    #[cfg(target_os = "android")]
    let dir_opt: Option<std::path::PathBuf> = Some(folder.into());
    if let Some(dir) = dir_opt {
        match dir.try_exists() {
            Ok(false) | Err(_) => {
                log::error!("The {dir:?} directory (constructed from the {folder} folder) was not found");
            }
            Ok(true) => {
                // The folder does exist
                let files = fs::read_dir(dir);
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
                        let file_content = fs::read_to_string(path);
                        if let Ok(file_content) = file_content {
                            files_data.push(FileData {
                                name: file_name,
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
