#[cfg(debug_assertions)]
pub const ENABLE_UPDATES_CHECKS: bool = true;
#[cfg(not(debug_assertions))]
pub const ENABLE_UPDATES_CHECKS: bool = false;

#[derive(serde::Deserialize)]
pub struct Config {
    pub content_server_url: String,
    pub main_server_url: String,
    pub discord_server_invite: String,
}

pub fn get_config() -> Config {
    let data = include_str!("../config.json");
    let res: Config = serde_json::from_str(data).expect("Unable to parse the configuration file.");
    log::info!("Successfully loaded the config file");
    res
}
