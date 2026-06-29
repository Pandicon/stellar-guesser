use crate::{action::Action, rendering::themes::Theme};

pub mod angular_separation;
pub mod find_this_object;
pub mod guess_ra_dec;
pub mod guess_the_magnitude;
pub mod mark_missing_object;
pub mod which_constellation_is_point_in;
pub mod which_object_is_here;
pub mod which_object_is_missing;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Settings {
    pub angular_separation: angular_separation::Settings,
    pub find_this_object: find_this_object::Settings,
    pub guess_rad_dec: guess_ra_dec::Settings,
    pub guess_the_magnitude: guess_the_magnitude::Settings,
    pub mark_missing_object: mark_missing_object::Settings,
    pub what_constellation_is_this_point_in: which_constellation_is_point_in::Settings,
    pub what_is_this_object: which_object_is_here::Settings,
    pub which_object_is_missing: which_object_is_missing::Settings,
}

#[allow(clippy::derivable_impls)]
impl Default for Settings {
    fn default() -> Self {
        Self {
            angular_separation: angular_separation::Settings::default(),
            find_this_object: find_this_object::Settings::default(),
            guess_rad_dec: guess_ra_dec::Settings::default(),
            guess_the_magnitude: guess_the_magnitude::Settings::default(),
            mark_missing_object: mark_missing_object::Settings::default(),
            what_constellation_is_this_point_in: which_constellation_is_point_in::Settings::default(),
            what_is_this_object: which_object_is_here::Settings::default(),
            which_object_is_missing: which_object_is_missing::Settings::default(),
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
    MarkMissingObject(mark_missing_object::SmallSettings),
    WhatIsThisObject(which_object_is_here::SmallSettings),
    WhichConstellationIsThisPointIn(which_constellation_is_point_in::SmallSettings),
    WhichObjectIsMissing(which_object_is_missing::SmallSettings),
}

pub enum Question {
    AngularSeparation(angular_separation::Question),
    FindThisObject(find_this_object::Question),
    GuessDec(guess_ra_dec::DecQuestion),
    GuessRa(guess_ra_dec::RaQuestion),
    GuessTheMagnitude(guess_the_magnitude::Question),
    MarkMissingObject(mark_missing_object::Question),
    WhatIsThisObject(which_object_is_here::Question),
    WhichConstellationIsThisPointIn(which_constellation_is_point_in::Question),
    WhichObjectIsMissing(which_object_is_missing::Question),
}

impl Question {
    pub fn activate(&self) -> ActiveQuestion {
        match self {
            Question::AngularSeparation(question) => ActiveQuestion::AngularSeparation(question.activate()),
            Question::FindThisObject(question) => ActiveQuestion::FindThisObject(question.activate()),
            Question::GuessDec(dec_question) => ActiveQuestion::GuessDec(dec_question.activate()),
            Question::GuessRa(ra_question) => ActiveQuestion::GuessRa(ra_question.activate()),
            Question::GuessTheMagnitude(question) => ActiveQuestion::GuessTheMagnitude(question.activate()),
            Question::MarkMissingObject(question) => ActiveQuestion::MarkMissingObject(question.activate()),
            Question::WhatIsThisObject(question) => ActiveQuestion::WhatIsThisObject(question.activate()),
            Question::WhichConstellationIsThisPointIn(question) => ActiveQuestion::WhichConstellationIsThisPointIn(question.activate()),
            Question::WhichObjectIsMissing(question) => ActiveQuestion::WhichObjectIsMissing(question.activate()),
        }
    }
}

pub enum ActiveQuestion {
    AngularSeparation(angular_separation::ActiveQuestion),
    FindThisObject(find_this_object::ActiveQuestion),
    GuessDec(guess_ra_dec::ActiveDecQuestion),
    GuessRa(guess_ra_dec::ActiveRaQuestion),
    GuessTheMagnitude(guess_the_magnitude::ActiveQuestion),
    MarkMissingObject(mark_missing_object::ActiveQuestion),
    WhatIsThisObject(which_object_is_here::ActiveQuestion),
    WhichConstellationIsThisPointIn(which_constellation_is_point_in::ActiveQuestion),
    WhichObjectIsMissing(which_object_is_missing::ActiveQuestion),
}

impl ActiveQuestion {
    pub fn should_display_input(&self) -> bool {
        match &self {
            ActiveQuestion::AngularSeparation(active_question) => active_question.should_display_input(),
            ActiveQuestion::FindThisObject(active_question) => active_question.should_display_input(),
            ActiveQuestion::GuessDec(active_question) => active_question.should_display_input(),
            ActiveQuestion::GuessRa(active_question) => active_question.should_display_input(),
            ActiveQuestion::GuessTheMagnitude(active_question) => active_question.should_display_input(),
            ActiveQuestion::MarkMissingObject(active_question) => active_question.should_display_input(),
            ActiveQuestion::WhatIsThisObject(active_question) => active_question.should_display_input(),
            ActiveQuestion::WhichConstellationIsThisPointIn(active_question) => active_question.should_display_input(),
            ActiveQuestion::WhichObjectIsMissing(active_question) => active_question.should_display_input(),
        }
    }

    pub fn show_circle_marker(&self) -> bool {
        match &self {
            ActiveQuestion::AngularSeparation(active_question) => active_question.show_circle_marker(),
            ActiveQuestion::FindThisObject(active_question) => active_question.show_circle_marker(),
            ActiveQuestion::GuessDec(active_question) => active_question.show_circle_marker(),
            ActiveQuestion::GuessRa(active_question) => active_question.show_circle_marker(),
            ActiveQuestion::GuessTheMagnitude(active_question) => active_question.show_circle_marker(),
            ActiveQuestion::MarkMissingObject(active_question) => active_question.show_circle_marker(),
            ActiveQuestion::WhatIsThisObject(active_question) => active_question.show_circle_marker(),
            ActiveQuestion::WhichConstellationIsThisPointIn(active_question) => active_question.show_circle_marker(),
            ActiveQuestion::WhichObjectIsMissing(active_question) => active_question.show_circle_marker(),
        }
    }

    pub fn show_tolerance_marker(&self) -> bool {
        match &self {
            ActiveQuestion::AngularSeparation(active_question) => active_question.show_tolerance_marker(),
            ActiveQuestion::FindThisObject(active_question) => active_question.show_tolerance_marker(),
            ActiveQuestion::GuessDec(active_question) => active_question.show_tolerance_marker(),
            ActiveQuestion::GuessRa(active_question) => active_question.show_tolerance_marker(),
            ActiveQuestion::GuessTheMagnitude(active_question) => active_question.show_tolerance_marker(),
            ActiveQuestion::MarkMissingObject(active_question) => active_question.show_tolerance_marker(),
            ActiveQuestion::WhatIsThisObject(active_question) => active_question.show_tolerance_marker(),
            ActiveQuestion::WhichConstellationIsThisPointIn(active_question) => active_question.show_tolerance_marker(),
            ActiveQuestion::WhichObjectIsMissing(active_question) => active_question.show_tolerance_marker(),
        }
    }

    pub fn get_question_distance_tolerance(&self) -> angle::Deg<f32> {
        match &self {
            ActiveQuestion::AngularSeparation(active_question) => active_question.get_question_distance_tolerance(),
            ActiveQuestion::FindThisObject(active_question) => active_question.get_question_distance_tolerance(),
            ActiveQuestion::GuessDec(active_question) => active_question.get_question_distance_tolerance(),
            ActiveQuestion::GuessRa(active_question) => active_question.get_question_distance_tolerance(),
            ActiveQuestion::GuessTheMagnitude(active_question) => active_question.get_question_distance_tolerance(),
            ActiveQuestion::MarkMissingObject(active_question) => active_question.get_question_distance_tolerance(),
            ActiveQuestion::WhatIsThisObject(active_question) => active_question.get_question_distance_tolerance(),
            ActiveQuestion::WhichConstellationIsThisPointIn(active_question) => active_question.get_question_distance_tolerance(),
            ActiveQuestion::WhichObjectIsMissing(active_question) => active_question.get_question_distance_tolerance(),
        }
    }

    pub fn allow_multiple_player_markers(&self) -> bool {
        match &self {
            ActiveQuestion::AngularSeparation(active_question) => active_question.allow_multiple_player_markers(),
            ActiveQuestion::FindThisObject(active_question) => active_question.allow_multiple_player_markers(),
            ActiveQuestion::GuessDec(active_question) => active_question.allow_multiple_player_markers(),
            ActiveQuestion::GuessRa(active_question) => active_question.allow_multiple_player_markers(),
            ActiveQuestion::GuessTheMagnitude(active_question) => active_question.allow_multiple_player_markers(),
            ActiveQuestion::MarkMissingObject(active_question) => active_question.allow_multiple_player_markers(),
            ActiveQuestion::WhatIsThisObject(active_question) => active_question.allow_multiple_player_markers(),
            ActiveQuestion::WhichConstellationIsThisPointIn(active_question) => active_question.allow_multiple_player_markers(),
            ActiveQuestion::WhichObjectIsMissing(active_question) => active_question.allow_multiple_player_markers(),
        }
    }

    pub fn add_marker_on_click(&self) -> bool {
        match &self {
            ActiveQuestion::AngularSeparation(active_question) => active_question.add_marker_on_click(),
            ActiveQuestion::FindThisObject(active_question) => active_question.add_marker_on_click(),
            ActiveQuestion::GuessDec(active_question) => active_question.add_marker_on_click(),
            ActiveQuestion::GuessRa(active_question) => active_question.add_marker_on_click(),
            ActiveQuestion::GuessTheMagnitude(active_question) => active_question.add_marker_on_click(),
            ActiveQuestion::MarkMissingObject(active_question) => active_question.add_marker_on_click(),
            ActiveQuestion::WhatIsThisObject(active_question) => active_question.add_marker_on_click(),
            ActiveQuestion::WhichConstellationIsThisPointIn(active_question) => active_question.add_marker_on_click(),
            ActiveQuestion::WhichObjectIsMissing(active_question) => active_question.add_marker_on_click(),
        }
    }

    pub fn start_question(&mut self, theme: &Theme, actions: &mut Vec<Action>) {
        match self {
            ActiveQuestion::AngularSeparation(active_question) => active_question.start_question(theme, actions),
            ActiveQuestion::FindThisObject(active_question) => active_question.start_question(theme, actions),
            ActiveQuestion::GuessDec(active_question) => active_question.start_question(theme, actions),
            ActiveQuestion::GuessRa(active_question) => active_question.start_question(theme, actions),
            ActiveQuestion::GuessTheMagnitude(active_question) => active_question.start_question(theme, actions),
            ActiveQuestion::MarkMissingObject(active_question) => active_question.start_question(theme, actions),
            ActiveQuestion::WhatIsThisObject(active_question) => active_question.start_question(theme, actions),
            ActiveQuestion::WhichConstellationIsThisPointIn(active_question) => active_question.start_question(theme, actions),
            ActiveQuestion::WhichObjectIsMissing(active_question) => active_question.start_question(theme, actions),
        }
    }
}

pub fn question_pack_to_string(name: &str, question_pack: &crate::game::questions_filter::QuestionPack) -> String {
    format!(
        "{}{}{}{}{}{}{}",
        name,
        crate::game::game_handler::QUESTION_PACK_PARTS_DIV,
        question_pack.query,
        crate::game::game_handler::QUESTION_PACK_PARTS_DIV,
        question_pack.description,
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
