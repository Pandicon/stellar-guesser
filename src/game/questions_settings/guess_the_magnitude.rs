#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GuessTheMagnitudeQuestionsSettings {
	pub magnitude_cutoff: f32,
	pub replay_incorrect: bool,
	pub show: bool,
}

impl Default for GuessTheMagnitudeQuestionsSettings {
	fn default() -> Self {
		Self {
			magnitude_cutoff: 6.0,
			replay_incorrect: true,
			show: true,
		}
	}
}
