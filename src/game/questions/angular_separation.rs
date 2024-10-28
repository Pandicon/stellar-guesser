use crate::game::game_handler::{self, GameHandler};
use crate::geometry;
use crate::renderer::CellestialSphere;
use crate::rendering::caspr::markers::game_markers::{GameMarker, GameMarkerType};
use crate::rendering::themes::Theme;
use angle::{Angle, Deg};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Settings {
    pub show: bool,
    pub rotate_to_midpoint: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self { show: true, rotate_to_midpoint: true }
    }
}

#[derive(Clone, Default)]
pub struct State {
    answer: String,
}

#[derive(Clone)]
pub struct Question {
    /// (ra, dec)
    pub point1: (angle::Deg<f32>, angle::Deg<f32>),
    /// (ra, dec)
    pub point2: (angle::Deg<f32>, angle::Deg<f32>),

    pub state: State,
}

impl Question {
    pub fn new_random() -> Self {
        Self {
            point1: geometry::generate_random_point(&mut rand::thread_rng()),
            point2: geometry::generate_random_point(&mut rand::thread_rng()),

            state: State::default(),
        }
    }
}

impl crate::game::game_handler::Question for Question {
    fn check_answer(&self, game_handler: &mut GameHandler, _cellestial_sphere: &mut CellestialSphere, _theme: &Theme) {
        let (ra1, dec1) = self.point1;
        let (ra2, dec2) = self.point2;
        let distance = geometry::angular_distance((ra1.to_rad(), dec1.to_rad()), (ra2.to_rad(), dec2.to_rad())).to_deg();
        match self.state.answer.parse::<f32>() {
            Ok(answer) => {
                let answer = angle::Deg(answer);
                game_handler.answer_review_text_heading = format!("You were {:.1} degrees away!", (distance - answer).value());
                let error_percent = 1.0 - answer.value() / distance.value();
                game_handler.answer_review_text = format!("The real distance was {:.1}°. Your error is equal to {:.1}% of the distance.", distance.value(), error_percent * 100.0);
                if game_handler.game_settings.is_scored_mode {
                    let error = (1.0 - answer.value() / distance.value()).abs();
                    if error < 0.03 {
                        game_handler.score += 3;
                    } else if error < 0.05 {
                        game_handler.score += 2;
                    } else if error < 0.1 {
                        game_handler.score += 1;
                    }
                    game_handler.increment_possible_score(3);
                }
            }
            Err(_) => {
                game_handler.answer_review_text_heading = "You didn't guess".to_string();
                game_handler.answer_review_text = format!("The real distance was {:.1}°.", distance);
            }
        };
        game_handler.use_up_current_question();
    }

    fn can_choose_as_next(&self, game_handler: &mut GameHandler) -> bool {
        game_handler.questions_settings.angular_separation.show
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
        let (ra1, dec1) = self.point1;
        let (ra2, dec2) = self.point2;
        cellestial_sphere.game_markers.markers = vec![
            GameMarker::new(GameMarkerType::Task, ra1, dec1, 2.0, 5.0, false, false, &theme.game_visuals.game_markers_colours),
            GameMarker::new(GameMarkerType::Task, ra2, dec2, 2.0, 5.0, false, false, &theme.game_visuals.game_markers_colours),
        ];
        if game_handler.questions_settings.angular_separation.rotate_to_midpoint {
            let end_1 = geometry::get_point_vector(ra1, dec1, &nalgebra::Matrix3::<f32>::identity());
            let end_2 = geometry::get_point_vector(ra2, dec2, &nalgebra::Matrix3::<f32>::identity());
            if (end_1 + end_2).magnitude_squared() > 10e-4 {
                let final_vector = (end_1 + end_2).normalize();
                cellestial_sphere.look_at_point(&final_vector);
                cellestial_sphere.init_renderers();
            }
        }
    }

    fn get_display_question(&self) -> String {
        String::from("What is the angular distance between these markers?")
    }

    fn clone_box(&self) -> Box<dyn game_handler::Question> {
        Box::new(self.clone())
    }
}
