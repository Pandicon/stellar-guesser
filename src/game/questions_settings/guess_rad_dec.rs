#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GuessRadDecQuestionsSettings {
	pub show: bool,
}

impl Default for GuessRadDecQuestionsSettings {
	fn default() -> Self {
		Self { show: true }
	}
}
