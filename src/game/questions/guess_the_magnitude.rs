use crate::game::game_handler::GameHandler;
use crate::renderer::CellestialSphere;
use crate::rendering::themes::Theme;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Settings {
    pub rotate_to_point: bool,
    pub magnitude_cutoff: f32,
    pub replay_incorrect: bool,
    pub show: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            rotate_to_point: true,
            magnitude_cutoff: 6.0,
            replay_incorrect: true,
            show: true,
        }
    }
}

pub struct Question {
    ra: angle::Deg<f32>,
    dec: angle::Deg<f32>,
    mag: f32,

    answer: String,
}

impl crate::game::game_handler::Question for Question {
    fn check_answer(&self, game_handler: &mut GameHandler, _cellestial_sphere: &mut CellestialSphere, _theme: &Theme) {
        match self.answer.parse::<f32>() {
            Ok(answer) => {
                let error = (self.mag - answer).abs();
                game_handler.answer_review_text_heading = format!("You were {:.1} mag away!", error);

                game_handler.answer_review_text = format!("The magnitude was {:.1}.", self.mag);

                if game_handler.game_settings.is_scored_mode {
                    if error < 0.3 {
                        game_handler.score += 3;
                    } else if error < 0.7 {
                        game_handler.score += 2;
                    } else if error < 1.5 {
                        game_handler.score += 1;
                    }
                    game_handler.increment_possible_score(3);
                }
            }
            Err(_) => {
                game_handler.answer_review_text_heading = "You didn't guess".to_string();
                game_handler.answer_review_text = format!("The magnitude was {:.1}.", self.mag);
            }
        };
        game_handler.use_up_current_question();
    }

    fn can_choose_as_next(&self, game_handler: &mut GameHandler) -> bool {
        game_handler.questions_settings.guess_the_magnitude.show && self.mag < game_handler.questions_settings.guess_the_magnitude.magnitude_cutoff
    }

    fn reset(self) -> Self {
        self
    }
}
