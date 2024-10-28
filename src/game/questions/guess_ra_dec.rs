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

pub struct RaQuestion {
    ra: angle::Deg<f32>,

    answer: String,
}

impl RaQuestion {
    pub fn new_random() -> Self {
        let (ra, _dec) = geometry::generate_random_point(&mut rand::thread_rng());
        Self { ra, answer: String::new() }
    }
}

impl crate::game::game_handler::Question for RaQuestion {
    fn check_answer(&self, game_handler: &mut GameHandler, _cellestial_sphere: &mut CellestialSphere, _theme: &Theme) {
        match self.answer.parse::<f32>() {
            Ok(answer_hours) => {
                let answer_deg = angle::Deg(answer_hours / 24.0 * 360.0);
                let error_deg = (self.ra - answer_deg).abs();
                game_handler.answer_review_text_heading = format!("You were {:.1}h away!", error_deg.value() / 360.0 * 24.0);

                game_handler.answer_review_text = format!("The real right ascension was {:.1}h", self.ra.value() / 360.0 * 24.0);

                if game_handler.game_settings.is_scored_mode {
                    if error_deg < angle::Deg(3.0) {
                        game_handler.score += 3;
                    } else if error_deg < angle::Deg(5.0) {
                        game_handler.score += 2;
                    } else if error_deg < angle::Deg(10.0) {
                        game_handler.score += 1;
                    }
                    game_handler.increment_possible_score(3);
                }
            }
            Err(_) => {
                game_handler.answer_review_text_heading = "You didn't guess".to_string();
                game_handler.answer_review_text = format!("The real right ascension was {:.1}h.", self.ra.value() / 360.0 * 24.0);
            }
        };
        game_handler.use_up_current_question();
    }

    fn can_choose_as_next(&self, game_handler: &mut GameHandler) -> bool {
        game_handler.questions_settings.guess_rad_dec.show
    }

    fn reset(self) -> Self {
        Self::new_random()
    }
}

pub struct DecQuestion {
    dec: angle::Deg<f32>,

    answer: String,
}

impl DecQuestion {
    pub fn new_random() -> Self {
        let (_ra, dec) = geometry::generate_random_point(&mut rand::thread_rng());
        Self { dec, answer: String::new() }
    }
}

impl crate::game::game_handler::Question for DecQuestion {
    fn check_answer(&self, game_handler: &mut GameHandler, _cellestial_sphere: &mut CellestialSphere, _theme: &Theme) {
        match self.answer.parse::<f32>() {
            Ok(answer) => {
                let answer_deg = angle::Deg(answer);
                let error = (self.dec - answer_deg).abs();
                game_handler.answer_review_text_heading = format!("You were {:.1}° away!", error.value());

                game_handler.answer_review_text = format!("The declination was {:.1}°", self.dec.value());

                if game_handler.game_settings.is_scored_mode {
                    if error < angle::Deg(3.0) {
                        game_handler.score += 3;
                    } else if error < angle::Deg(5.0) {
                        game_handler.score += 2;
                    } else if error < angle::Deg(10.0) {
                        game_handler.score += 1;
                    }
                    game_handler.increment_possible_score(3);
                }
            }
            Err(_) => {
                game_handler.answer_review_text_heading = "You didn't guess".to_string();
                game_handler.answer_review_text = format!("The declination was {:.1}°.", self.dec);
            }
        };
        game_handler.use_up_current_question();
    }

    fn can_choose_as_next(&self, game_handler: &mut GameHandler) -> bool {
        game_handler.questions_settings.guess_rad_dec.show
    }

    fn reset(self) -> Self {
        Self::new_random()
    }
}
