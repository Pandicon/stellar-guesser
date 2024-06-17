#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
pub const OBJECT_IMAGES_ADDON_FOLDER: &str = "./addons/object-images";
#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
pub const THEMES_FOLDER: &str = "./addons/themes";
#[cfg(target_os = "android")]
pub const OBJECT_IMAGES_ADDON_FOLDER: &str = "/storage/emulated/0/Android/data/com.github.noreply.users.stellar_guesser/files/addons/object-images"; // For whatever reason the Documents folder wouldn't work...
#[cfg(target_os = "android")]
pub const THEMES_FOLDER: &str = "/storage/emulated/0/Android/data/com.github.noreply.users.stellar_guesser/files/addons/themes";
pub const ANDROID_PACKAGE_NAME: &str = "com.github.noreply.users.stellar_guesser";
pub const DESKTOP_PACKAGE_NAME: &str = "stellar_guesser";
