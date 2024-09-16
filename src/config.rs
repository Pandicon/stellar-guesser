#[derive(serde::Deserialize)]
pub struct Config {
	pub content_server_url: String,
	pub main_server_url: String,
}

pub fn get_config() -> Config {
	let path = "./config.json";
	let data = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Unable to read the {:?} file.", path));
	let res: Config = serde_json::from_str(&data).expect("Unable to parse the configuration file.");
	log::info!("Successfully loaded the config file");
	res
}