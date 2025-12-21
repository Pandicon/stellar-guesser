include!(concat!(env!("OUT_DIR"), "/embedded_loader.rs"));

fn read_file_embedded(raw_path: impl Into<std::path::PathBuf>) -> Result<crate::files_handling::FileInfo, std::io::Error> {
    let path_buf = raw_path.into();
    let path_obj = std::path::Path::new(&path_buf);

    let clean_path = path_obj.components().as_path();

    if let Some(path_str) = clean_path.to_str() {
        let embedded_key = path_str.trim_start_matches("./").trim_start_matches(".\\").trim_start_matches("data/");

        match EMBEDDED_DATA.get_file(embedded_key) {
            Some(file) => {
                let contents = file.contents().to_vec();
                let name = clean_path.file_name().map(|n| n.to_str()).flatten().map(|c| c.to_owned());
                Ok(super::FileInfo {
                    name,
                    path: clean_path.into(),
                    contents,
                })
            }
            None => Err(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found")),
        }
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::InvalidFilename, "File name is not valid"))
    }
}

pub fn read_file_relative(file_path: impl Into<std::path::PathBuf>) -> Result<super::FileInfo, std::io::Error> {
    let file_path = file_path.into();
    match super::read_file_relative_filesystem(crate::config::BASE_PATH, &file_path) {
        Ok(r) => Ok(r),
        Err(_) => {
            log::debug!("Did not find {:?} in the filesystem, falling back to the embedded directory", file_path.to_str());
            read_file_embedded(file_path)
        }
    }
}

fn read_dir_embedded(raw_path: impl Into<std::path::PathBuf>) -> Result<Vec<crate::files_handling::FileInfo>, std::io::Error> {
    let path_buf = raw_path.into();
    let path_obj = std::path::Path::new(&path_buf);

    let clean_path = path_obj.components().as_path();

    if let Some(path_str) = clean_path.to_str() {
        let embedded_key = path_str.trim_start_matches("./").trim_start_matches(".\\").trim_start_matches("data/");

        match EMBEDDED_DATA.get_dir(embedded_key) {
            Some(directory) => Ok(directory
                .files()
                .filter_map(|f| match read_file_embedded(f.path()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        log::error!("Could not read file {:?}: {}", f.path(), err);
                        None
                    }
                })
                .collect()),
            None => Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Directory not found")),
        }
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::InvalidFilename, "Directory name is not valid"))
    }
}

pub fn read_dir_relative(relative_path: impl Into<std::path::PathBuf>) -> Result<Vec<crate::files_handling::FileInfo>, std::io::Error> {
    let relative_path = relative_path.into();
    let embedded_res = read_dir_embedded(&relative_path);
    let fs_res = super::read_dir_relative_filesystem(crate::config::BASE_PATH, &relative_path);

    let mut files = Vec::new();

    match embedded_res {
        Ok(em) => files.extend(em),
        Err(e) => log::error!("Could not find the files in the embedded directory {:?}: {e}", relative_path.to_str()),
    }

    match fs_res {
        Ok(fi) => {
            for fs_file in fi {
                // Remove any embedded file that has the same name as a filesystem file
                if let Some(name) = &fs_file.name {
                    files.retain(|em_file| em_file.name.as_ref() != Some(name));
                }
                files.push(fs_file);
            }
        }
        Err(e) => log::error!("Could not find the files in the filesystem directory {:?}: {e}", relative_path.to_str()),
    }

    Ok(files)
}

pub fn get_path_relative(relative_path: impl Into<std::path::PathBuf>) -> Result<std::path::PathBuf, std::io::Error> {
    Ok(super::get_path_relative_inner(crate::config::BASE_PATH, relative_path))
}
