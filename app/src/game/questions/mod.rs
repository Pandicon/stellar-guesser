pub mod angular_separation;
pub mod find_this_object;
pub mod guess_ra_dec;
pub mod guess_the_magnitude;
pub mod which_constellation_is_point_in;
pub mod which_object_is_here;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Settings {
    pub angular_separation: angular_separation::Settings,
    pub find_this_object: find_this_object::Settings,
    pub guess_rad_dec: guess_ra_dec::Settings,
    pub guess_the_magnitude: guess_the_magnitude::Settings,
    pub what_constellation_is_this_point_in: which_constellation_is_point_in::Settings,
    pub what_is_this_object: which_object_is_here::Settings,
}

#[allow(clippy::derivable_impls)]
impl Default for Settings {
    fn default() -> Self {
        Self {
            angular_separation: angular_separation::Settings::default(),
            find_this_object: find_this_object::Settings::default(),
            guess_rad_dec: guess_ra_dec::Settings::default(),
            guess_the_magnitude: guess_the_magnitude::Settings::default(),
            what_constellation_is_this_point_in: which_constellation_is_point_in::Settings::default(),
            what_is_this_object: which_object_is_here::Settings::default(),
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub enum QuestionType {
    AngularSeparation(angular_separation::SmallSettings),
    FindThisObject(find_this_object::SmallSettings),
    GuessDec(guess_ra_dec::SmallSettings),
    GuessRa(guess_ra_dec::SmallSettings),
    GuessTheMagnitude(guess_the_magnitude::SmallSettings),
    WhatIsThisObject(which_object_is_here::SmallSettings),
    WhichConstellationIsThisPointIn(which_constellation_is_point_in::SmallSettings),
}

pub fn question_pack_to_string(name: &str, question_pack: &crate::game::questions_filter::QuestionPack) -> String {
    format!(
        "{}{}{}{}{}",
        name,
        crate::game::game_handler::QUESTION_PACK_PARTS_DIV,
        question_pack.query,
        crate::game::game_handler::QUESTION_PACK_PARTS_DIV,
        question_pack
            .question_objects
            .iter()
            .filter_map(|(settings, object_ids)| {
                match serde_json::to_string(settings) {
                    Ok(string) => Some(format!(
                        "{}{}{}",
                        string,
                        crate::game::game_handler::QUESTION_PACK_QUESTIONS_PARTS_DIV,
                        object_ids.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(",")
                    )),
                    Err(err) => {
                        log::error!("Failed to serialize question pack settings: {:?}", err);
                        None
                    }
                }
            })
            .collect::<Vec<String>>()
            .join(crate::game::game_handler::QUESTION_PACK_QUESTIONS_DIV)
    )
}
