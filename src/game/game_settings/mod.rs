#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GameSettings {
	pub no_of_questions: u32,
	pub is_scored_mode: bool,
}

impl Default for GameSettings {
	fn default() -> Self {
		Self {
			no_of_questions: 15,
			is_scored_mode: false,
		}
	}
}
