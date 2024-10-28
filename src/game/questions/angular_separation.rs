use crate::game::game_handler::GameHandler;
use crate::geometry;
use crate::renderer::CellestialSphere;
use crate::rendering::themes::Theme;
use angle::Angle;

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

pub struct Question {
    /// (ra, dec)
    point1: (angle::Deg<f32>, angle::Deg<f32>),
    /// (ra, dec)
    point2: (angle::Deg<f32>, angle::Deg<f32>),

    answer: String,
}

impl Question {
    pub fn new_random() -> Self {
        Self {
            point1: geometry::generate_random_point(&mut rand::thread_rng()),
            point2: geometry::generate_random_point(&mut rand::thread_rng()),

            answer: String::new(),
        }
    }
}

impl crate::game::game_handler::Question for Question {
    fn check_answer(&self, game_handler: &mut GameHandler, _cellestial_sphere: &mut CellestialSphere, _theme: &Theme) {
        let (ra1, dec1) = self.point1;
        let (ra2, dec2) = self.point2;
        let distance = geometry::angular_distance((ra1.to_rad(), dec1.to_rad()), (ra2.to_rad(), dec2.to_rad())).to_deg();
        match self.answer.parse::<f32>() {
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

    fn reset(self) -> Self {
        Self::new_random()
    }
}
