pub mod angular_separation;
pub mod find_this_object;
pub mod guess_rad_dec;
pub mod guess_the_magnitude;
pub mod what_constellation_is_this_point_in;
pub mod what_is_this_object;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct QuestionsSettings {
    pub angular_separation: angular_separation::AngularSeparationQuestionsSettings,
    pub find_this_object: find_this_object::FindThisObjectQuestionsSettings,
    pub guess_rad_dec: guess_rad_dec::GuessRadDecQuestionsSettings,
    pub guess_the_magnitude: guess_the_magnitude::GuessTheMagnitudeQuestionsSettings,
    pub what_constellation_is_this_point_in: what_constellation_is_this_point_in::WhatConstellationIsThisPointInQuestionsSettings,
    pub what_is_this_object: what_is_this_object::WhatIsThisObjectQuestionsSettings,
}

impl Default for QuestionsSettings {
    fn default() -> Self {
        Self {
            angular_separation: angular_separation::AngularSeparationQuestionsSettings::default(),
            find_this_object: find_this_object::FindThisObjectQuestionsSettings::default(),
            guess_rad_dec: guess_rad_dec::GuessRadDecQuestionsSettings::default(),
            guess_the_magnitude: guess_the_magnitude::GuessTheMagnitudeQuestionsSettings::default(),
            what_constellation_is_this_point_in: what_constellation_is_this_point_in::WhatConstellationIsThisPointInQuestionsSettings::default(),
            what_is_this_object: what_is_this_object::WhatIsThisObjectQuestionsSettings::default(),
        }
    }
}
