#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WhatConstellationIsThisPointInQuestionsSettings {
    pub show: bool,
    pub rotate_to_point: bool,
}

impl Default for WhatConstellationIsThisPointInQuestionsSettings {
    fn default() -> Self {
        Self { show: true, rotate_to_point: true }
    }
}
