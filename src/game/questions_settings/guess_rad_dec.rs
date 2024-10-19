#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GuessRadDecQuestionsSettings {
    pub show: bool,
    pub rotate_to_point: bool,
}

impl Default for GuessRadDecQuestionsSettings {
    fn default() -> Self {
        Self { show: true, rotate_to_point: true }
    }
}
