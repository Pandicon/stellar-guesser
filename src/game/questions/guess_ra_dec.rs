use crate::game::game_handler;
use crate::game::game_handler::GameHandler;
use crate::geometry;
use crate::renderer::CellestialSphere;
use crate::rendering::caspr::markers::game_markers::{GameMarker, GameMarkerType};
use crate::rendering::themes::Theme;
use angle::{Angle, Deg};

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

#[derive(Clone, Default)]
pub struct State {
    answer: String,
}

#[derive(Clone)]
pub struct RaQuestion {
    pub dec: angle::Deg<f32>,
    pub ra: angle::Deg<f32>,

    pub state: State,
}

impl RaQuestion {
    pub fn new_random() -> Self {
        let (ra, dec) = geometry::generate_random_point(&mut rand::thread_rng());
        Self { dec, ra, state: State::default() }
    }
}

impl crate::game::game_handler::Question for RaQuestion {
    fn check_answer(&self, game_handler: &mut GameHandler, _cellestial_sphere: &mut CellestialSphere, _theme: &Theme) {
        match self.state.answer.parse::<f32>() {
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

    fn reset(self) -> Box<dyn game_handler::Question> {
        Box::new(Self::new_random())
    }

    fn show_tolerance_marker(&self) -> bool {
        false
    }

    fn show_circle_marker(&self) -> bool {
        false
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
            false,
            false,
            &theme.game_visuals.game_markers_colours,
        )];
        if game_handler.questions_settings.guess_rad_dec.rotate_to_point {
            let final_vector = geometry::get_point_vector(self.ra, self.dec, &nalgebra::Matrix3::<f32>::identity());
            cellestial_sphere.look_at_point(&final_vector);
            cellestial_sphere.init_renderers();
        }
    }

    fn get_display_question(&self) -> String {
        String::from("What is the right ascension of this point?")
    }

    fn clone_box(&self) -> Box<dyn game_handler::Question> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct DecQuestion {
    pub dec: angle::Deg<f32>,
    pub ra: angle::Deg<f32>,

    pub state: State,
}

impl DecQuestion {
    pub fn new_random() -> Self {
        let (ra, dec) = geometry::generate_random_point(&mut rand::thread_rng());
        Self { dec, ra, state: State::default() }
    }
}

impl crate::game::game_handler::Question for DecQuestion {
    fn check_answer(&self, game_handler: &mut GameHandler, _cellestial_sphere: &mut CellestialSphere, _theme: &Theme) {
        match self.state.answer.parse::<f32>() {
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

    fn reset(self) -> Box<dyn game_handler::Question> {
        Box::new(Self::new_random())
    }

    fn show_tolerance_marker(&self) -> bool {
        false
    }

    fn show_circle_marker(&self) -> bool {
        false
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
            false,
            false,
            &theme.game_visuals.game_markers_colours,
        )];
        if game_handler.questions_settings.guess_rad_dec.rotate_to_point {
            let final_vector = geometry::get_point_vector(self.ra, self.dec, &nalgebra::Matrix3::<f32>::identity());
            cellestial_sphere.look_at_point(&final_vector);
            cellestial_sphere.init_renderers();
        }
    }

    fn get_display_question(&self) -> String {
        String::from("What is the declination of this point?")
    }

    fn clone_box(&self) -> Box<dyn game_handler::Question> {
        Box::new(self.clone())
    }
}
