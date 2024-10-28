#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AngularSeparationQuestionsSettings {
    pub show: bool,
    pub rotate_to_midpoint: bool,
}

impl Default for AngularSeparationQuestionsSettings {
    fn default() -> Self {
        Self { show: true, rotate_to_midpoint: true }
    }
}
