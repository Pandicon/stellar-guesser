#[cfg(target_os = "android")]
use crate::ANDROID_PACKAGE_NAME;
#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
use crate::DESKTOP_PACKAGE_NAME;

use std::{
	collections::HashMap,
	io::Write,
	path::{Path, PathBuf},
};

// Android paths:
// - <package_name> is the package name, for example
// - /data/data/<packagename>/files/<path> (for example /data/data/<packagename>/files/id.txt) is a sandboxed piece of storage that no other app can access (and also the user can not access it without a rooted device)
// - /storage/emulated/0/Android/data/<packagename>/files/<path> (for example /storage/emulated/0/Android/data/<packagename>/files/id.txt) is a storage accessible by the user, but only from the computer as of newer Android versions
// - /storage/emulated/0/Documents/<path> (for example /storage/emulated/0/Documents/id.txt) is a completely public piece of storage in the Documents directory
pub struct Storage {
	pub cache: HashMap<String, String>,
	file_path: PathBuf,
	last_save_join_handle: Option<std::thread::JoinHandle<()>>,
	unsaved_changes: bool,
}

impl Storage {
	pub fn new() -> Self {
		#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
		let default_path = {
			let mut def_path = directories_next::ProjectDirs::from("", "", DESKTOP_PACKAGE_NAME)
				.map(|proj_dirs| proj_dirs.data_dir().to_path_buf())
				.unwrap_or(".".into());
			def_path.push("save.ron");
			def_path
		};
		#[cfg(target_os = "android")]
		let default_path = format!("/storage/emulated/0/Android/data/{ANDROID_PACKAGE_NAME}/files/save.ron");
		Self::with_file_path(default_path)
	}

	pub fn with_file_path(path: impl Into<PathBuf>) -> Self {
		let file_path: PathBuf = path.into();
		Self {
			cache: read_ron_file(&file_path).unwrap_or_default(),
			file_path,
			last_save_join_handle: None,
			unsaved_changes: false,
		}
	}

	pub fn get_string(&self, query: &str) -> Option<String> {
		self.cache.get(query).cloned()
	}

	pub fn set_string(&mut self, query: &str, data: String) {
		if self.cache.get(query) != Some(&data) {
			self.cache.insert(query.to_owned(), data);
			self.unsaved_changes = true;
		}
	}

	pub fn save(&mut self) {
		if self.unsaved_changes {
			self.unsaved_changes = false;

			if let Some(last_save_join_handle) = self.last_save_join_handle.take() {
				// Wait for the last save to finish
				last_save_join_handle.join().ok();
			}

			let file_path = self.file_path.clone();
			let data = self.cache.clone();

			let res = std::thread::Builder::new().name("saving_save_file".to_owned()).spawn(move || {
				save_to_disk(&file_path, &data);
			});
			match res {
				Ok(join_handle) => {
					self.last_save_join_handle = Some(join_handle);
				}
				Err(err) => {
					log::warn!("Failed to spawn a thread for saving the app state: {err}");
				}
			}
		}
	}
}

impl Drop for Storage {
	fn drop(&mut self) {
		if let Some(last_save_join_handle) = self.last_save_join_handle.take() {
			last_save_join_handle.join().ok();
		}
	}
}

fn read_ron_file<T: serde::de::DeserializeOwned>(file_path: impl AsRef<Path>) -> Option<T> {
	match std::fs::File::open(file_path) {
		Ok(file) => {
			let reader = std::io::BufReader::new(file);
			match ron::de::from_reader(reader) {
				Ok(value) => Some(value),
				Err(err) => {
					log::warn!("Failed to parse the RON save file: {}", err);
					None
				}
			}
		}
		Err(_err) => {
			// The file probably doesn't exist
			None
		}
	}
}

fn save_to_disk(file_path: &PathBuf, data: &HashMap<String, String>) {
	if let Some(parent_dir) = file_path.parent() {
		if !parent_dir.exists() {
			if let Err(err) = std::fs::create_dir_all(parent_dir) {
				log::warn!("Failed to create the parent directory {parent_dir:?}: {err}");
			}
		}
	}

	match std::fs::File::create(file_path) {
		Ok(file) => {
			let mut writer = std::io::BufWriter::new(file);
			let config = Default::default();

			if let Err(err) = ron::ser::to_writer_pretty(&mut writer, &data, config).and_then(|_| writer.flush().map_err(|err| err.into())) {
				log::warn!("Failed to serialize save data: {}", err);
			} else {
				log::trace!("Saved to {:?}", file_path);
			}
		}
		Err(err) => {
			log::warn!("Failed to create file {file_path:?}: {err}");
		}
	}
}
