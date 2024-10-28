use crate::game::game_handler;
use crate::game::game_handler::GameHandler;
use crate::geometry;
use crate::renderer::CellestialSphere;
use crate::rendering::caspr::markers::game_markers::{GameMarker, GameMarkerType};
use crate::rendering::themes::Theme;
use angle::Deg;

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

#[derive(Clone, Default)]
pub struct State {
    answer: String,
}

#[derive(Clone)]
pub struct Question {
    pub ra: angle::Deg<f32>,
    pub dec: angle::Deg<f32>,
    pub mag: f32,

    pub state: State,
}

impl crate::game::game_handler::Question for Question {
    fn check_answer(&self, game_handler: &mut GameHandler, _cellestial_sphere: &mut CellestialSphere, _theme: &Theme) {
        match self.state.answer.parse::<f32>() {
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

    fn reset(self) -> Box<dyn game_handler::Question> {
        Box::new(Self {
            ra: self.ra,
            dec: self.dec,
            mag: self.mag,

            state: State::default(),
        })
    }

    fn show_tolerance_marker(&self) -> bool {
        false
    }

    fn show_circle_marker(&self) -> bool {
        true
    }

    fn get_question_distance_tolerance(&self, _game_handler: &GameHandler) -> Deg<f32> {
        angle::Deg(0.0)
    }

    fn allow_multiple_player_markers(&self) -> bool {
        false
    }

    fn add_marker_on_click(&self) -> bool {
        false
    }

    fn should_display_input(&self) -> bool {
        true
    }

    fn start_question(&self, game_handler: &mut GameHandler, cellestial_sphere: &mut CellestialSphere, theme: &Theme) {
        cellestial_sphere.game_markers.markers = vec![GameMarker::new(
            GameMarkerType::Task,
            self.ra,
            self.dec,
            2.0,
            5.0,
            true,
            false,
            &theme.game_visuals.game_markers_colours,
        )];
        if game_handler.questions_settings.guess_the_magnitude.rotate_to_point {
            let final_vector = geometry::get_point_vector(self.ra, self.dec, &nalgebra::Matrix3::<f32>::identity());
            cellestial_sphere.look_at_point(&final_vector);
            cellestial_sphere.init_renderers();
        }
    }

    fn get_display_question(&self) -> String {
        String::from("What is the magnitude of this star?")
    }

    fn clone_box(&self) -> Box<dyn game_handler::Question> {
        Box::new(self.clone())
    }
}
