pub const BASE_PATH: &str = const_format::concatcp!("/storage/emulated/0/Android/data/", crate::config::ANDROID_PACKAGE_NAME, "/files");

pub const OBJECT_IMAGES_ADDON_FOLDER: &str = const_format::concatcp!("/storage/emulated/0/Android/data/", crate::config::ANDROID_PACKAGE_NAME, "/files/addons/object-images"); // For whatever reason the Documents folder wouldn't work...
pub const THEMES_FOLDER: &str = const_format::concatcp!("/storage/emulated/0/Android/data/", crate::config::ANDROID_PACKAGE_NAME, "/files/addons/themes");
pub const QUESTION_PACKS_FOLDER: &str = const_format::concatcp!("/storage/emulated/0/Android/data/", crate::config::ANDROID_PACKAGE_NAME, "/files/addons/question-packs");

pub const FOLDER_TO_EMBED: &str = "data";
