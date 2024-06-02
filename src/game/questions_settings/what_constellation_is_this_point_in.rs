#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WhatConstellationIsThisPointInQuestionsSettings {
	pub show: bool,
}

impl Default for WhatConstellationIsThisPointInQuestionsSettings {
	fn default() -> Self {
		Self { show: true }
	}
}
