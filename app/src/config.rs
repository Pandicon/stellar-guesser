#[derive(serde::Deserialize)]
pub struct Config {
    pub content_server_url: String,
    pub main_server_url: String,
}

pub fn get_config() -> Config {
    let data = include_str!("../config.json");
    let res: Config = serde_json::from_str(data).expect("Unable to parse the configuration file.");
    log::info!("Successfully loaded the config file");
    res
}
