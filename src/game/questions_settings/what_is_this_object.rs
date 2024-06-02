#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WhatIsThisObjectQuestionsSettings {
    pub show_messiers: bool,
    pub show_caldwells: bool,
    pub show_ngcs: bool,
    pub show_ics: bool,
    pub show_bayer: bool,
    pub show_starnames: bool,
    pub magnitude_cutoff: f32,
    pub correctness_threshold: f32,
    pub replay_incorrect: bool,
    pub show: bool,
}

impl Default for WhatIsThisObjectQuestionsSettings {
    fn default() -> Self {
        Self {
            show_messiers: true,
            show_caldwells: true,
            show_ngcs: true,
            show_ics: true,
            show_bayer: true,
            show_starnames: true,
            magnitude_cutoff: 6.0,
            correctness_threshold: 0.2,
            replay_incorrect: true,
            show: true,
        }
    }
}
