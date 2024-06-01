#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AngularSeparationQuestionsSettings {
	pub show: bool,
}

impl Default for AngularSeparationQuestionsSettings {
	fn default() -> Self {
		Self { show: true }
	}
}
