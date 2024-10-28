use crate::game::game_handler::GameHandler;
use crate::renderer::CellestialSphere;
use crate::rendering::themes::Theme;
use rand::Rng;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Settings {
    pub rotate_to_point: bool,
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

impl Default for Settings {
    fn default() -> Self {
        Self {
            rotate_to_point: true,
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

pub struct Question {
    possible_names: Vec<String>,
    ra: angle::Deg<f32>,
    dec: angle::Deg<f32>,
    is_messier: bool,
    is_caldwell: bool,
    is_ngc: bool,
    is_ic: bool,
    is_bayer: bool,
    is_starname: bool,
    magnitude: Option<f32>,
    object_type: String,
    constellation_abbreviation: String,
    images: Vec<crate::structs::image_info::ImageInfo>,

    answer: String,
}

impl crate::game::game_handler::Question for Question {
    fn check_answer(&self, game_handler: &mut GameHandler, _cellestial_sphere: &mut CellestialSphere, _theme: &Theme) {
        if !self.images.is_empty() {
            game_handler.answer_image = Some(self.images[rand::thread_rng().gen_range(0..self.images.len())].clone());
        }
        let possible_names_edited = self.possible_names.iter().map(|name| name.replace(' ', "").to_lowercase()).collect::<Vec<String>>();
        let correct = possible_names_edited.contains(&self.answer.replace(' ', "").to_lowercase());
        game_handler.answer_review_text_heading = format!(
            "{}orrect!",
            if correct {
                game_handler.score += 1;
                "C"
            } else {
                "Inc"
            }
        );
        game_handler.answer_review_text = format!(
            "Your answer was: {}\nPossible answers: {}\nObject type: {}",
            self.answer,
            self.possible_names.join(", "),
            self.object_type
        );
        game_handler.increment_possible_score(1);
        if !game_handler.questions_settings.what_is_this_object.replay_incorrect || correct {
            game_handler.use_up_current_question();
        } else {
            game_handler.question_number += 1;
        }
    }

    fn can_choose_as_next(&self, game_handler: &mut GameHandler) -> bool {
        let mag = (self.magnitude).unwrap_or(-1.0);
        game_handler.questions_settings.what_is_this_object.show
            && ((game_handler.questions_settings.what_is_this_object.show_messiers && self.is_messier)
                || (game_handler.questions_settings.what_is_this_object.show_caldwells && self.is_caldwell)
                || (game_handler.questions_settings.what_is_this_object.show_ngcs && self.is_ngc)
                || (game_handler.questions_settings.what_is_this_object.show_ics && self.is_ic)
                || (game_handler.questions_settings.what_is_this_object.show_bayer && self.is_bayer)
                || (game_handler.questions_settings.what_is_this_object.show_starnames && self.is_starname))
            && ((!self.is_bayer && !self.is_starname) || mag < game_handler.questions_settings.what_is_this_object.magnitude_cutoff)
            && *game_handler.active_constellations.entry(self.constellation_abbreviation.to_lowercase()).or_insert(true)
    }

    fn reset(self) -> Self {
        self
    }
}
