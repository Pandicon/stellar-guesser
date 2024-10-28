use crate::game::game_handler::GameHandler;
use crate::geometry;
use crate::renderer::CellestialSphere;
use crate::rendering::themes::Theme;
use angle::Angle;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Settings {
    pub show: bool,
    pub rotate_to_point: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self { show: true, rotate_to_point: true }
    }
}

pub struct Question {
    ra: angle::Deg<f32>,
    dec: angle::Deg<f32>,

    answer: String,
}

impl Question {
    pub fn new_random() -> Self {
        let (ra, dec) = geometry::generate_random_point(&mut rand::thread_rng());
        Self { ra, dec, answer: String::new() }
    }
}

impl crate::game::game_handler::Question for Question {
    fn check_answer(&self, game_handler: &mut GameHandler, cellestial_sphere: &mut CellestialSphere, _theme: &Theme) {
        let possible_abbrevs = cellestial_sphere.determine_constellation((self.ra.to_rad(), self.dec.to_rad()));
        let mut possible_constellation_names = Vec::new();
        for abbrev in possible_abbrevs {
            if let Some(constellation) = cellestial_sphere.constellations.get(&abbrev) {
                possible_constellation_names.extend(constellation.possible_names.iter().map(|name| name.replace(' ', "").to_lowercase()));
            };
        }
        let correct = possible_constellation_names.contains(&self.answer.replace(' ', "").to_lowercase());
        game_handler.answer_review_text_heading = format!(
            "{}orrect!",
            if correct {
                game_handler.score += 1;
                "C"
            } else {
                "Inc"
            }
        );
        game_handler.increment_possible_score(1);
        game_handler.answer_review_text = format!("Your answer was: {}\nThe right answers were: {}", self.answer, possible_constellation_names.join(", "));
        game_handler.use_up_current_question();
    }

    fn can_choose_as_next(&self, game_handler: &mut GameHandler) -> bool {
        game_handler.questions_settings.what_constellation_is_this_point_in.show
    }

    fn reset(self) -> Self {
        Self::new_random()
    }
}
