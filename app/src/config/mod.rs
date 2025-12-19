#[cfg(target_os = "android")]
mod android;
#[cfg(target_os = "android")]
pub use android::*;
#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
mod desktop;
#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
pub use desktop::*;

#[cfg(debug_assertions)]
pub const ENABLE_UPDATES_CHECKS: bool = true;
#[cfg(not(debug_assertions))]
pub const ENABLE_UPDATES_CHECKS: bool = false;

#[cfg(debug_assertions)]
pub const LOGGING_FILTER_LEVEL: log::LevelFilter = log::LevelFilter::Debug;
#[cfg(not(debug_assertions))]
pub const LOGGING_FILTER_LEVEL: log::LevelFilter = log::LevelFilter::Warn;

pub const ANDROID_PACKAGE_NAME: &str = "com.github.noreply.users.stellar_guesser";
pub const DESKTOP_PACKAGE_NAME: &str = "stellar_guesser";

#[derive(serde::Deserialize)]
pub struct Config {
    pub content_server_url: String,
    pub main_server_url: String,
    pub discord_server_invite: String,
}

pub fn get_config() -> Config {
    let data = include_str!("../../config.json");
    let res: Config = serde_json::from_str(data).expect("Unable to parse the configuration file.");
    log::info!("Successfully loaded the config file");
    res
}
