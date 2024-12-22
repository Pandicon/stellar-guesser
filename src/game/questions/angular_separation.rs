use crate::enums::GameStage;
use crate::game::game_handler::{self, GameHandler, QuestionCheckingData, QuestionTrait, QuestionWindowData};
use crate::game::questions;
use crate::renderer::CellestialSphere;
use crate::rendering::caspr::markers::game_markers::{GameMarker, GameMarkerType};
use crate::rendering::themes::Theme;
use angle::{Angle, Deg};
use eframe::egui;
use std::collections::HashMap;

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

    answer_review_text_heading: String,
    answer_review_text: String,
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
            point1: sg_geometry::generate_random_point(&mut rand::thread_rng()),
            point2: sg_geometry::generate_random_point(&mut rand::thread_rng()),

            state: State::default(),
        }
    }

    fn render_question_window(&mut self, data: QuestionWindowData) -> Option<egui::InnerResponse<Option<()>>> {
        egui::Window::new("Question").open(data.game_question_opened).show(data.ctx, |ui| {
            ui.heading(self.get_display_question());
            if self.should_display_input() {
                let text_input_response = ui.text_edit_singleline(&mut self.state.answer);
                if *data.request_input_focus {
                    text_input_response.request_focus();
                    *data.request_input_focus = false;
                }
                *data.input_field_has_focus |= text_input_response.has_focus();
            }
            if ui.button("Check").clicked() {
                self.check_answer(QuestionCheckingData {
                    cellestial_sphere: data.cellestial_sphere,
                    theme: data.theme,
                    game_stage: data.game_stage,
                    score: data.score,
                    possible_score: data.possible_score,
                    is_scored_mode: data.is_scored_mode,
                    current_question: data.current_question,
                    used_questions: data.used_questions,
                    add_marker_on_click: data.add_marker_on_click,
                    questions_settings: data.questions_settings,
                    question_number: data.question_number,
                    start_next_question: data.start_next_question,
                });
            }
            ui.label(data.question_number_text);
        })
    }

    fn render_answer_review_window(&self, data: QuestionWindowData) -> Option<egui::InnerResponse<Option<()>>> {
        egui::Window::new("Question").open(data.game_question_opened).show(data.ctx, |ui| {
            if !self.state.answer_review_text_heading.is_empty() {
                ui.heading(&self.state.answer_review_text_heading);
            }
            ui.label(&self.state.answer_review_text);
            if ui.button("Next").clicked() {
                *data.start_next_question = true;
            }
            ui.label(data.question_number_text);
        })
    }

    fn check_answer(&mut self, data: QuestionCheckingData) {
        let (ra1, dec1) = self.point1;
        let (ra2, dec2) = self.point2;
        let distance = sg_geometry::angular_distance((ra1.to_rad(), dec1.to_rad()), (ra2.to_rad(), dec2.to_rad())).to_deg();
        match self.state.answer.parse::<f32>() {
            Ok(answer) => {
                let answer = angle::Deg(answer);
                self.state.answer_review_text_heading = format!("You were {:.1} degrees away!", (distance - answer).value());
                let error_percent = 1.0 - answer.value() / distance.value();
                self.state.answer_review_text = format!("The real distance was {:.1}°. Your error is equal to {:.1}% of the distance.", distance.value(), error_percent * 100.0);
                if data.is_scored_mode {
                    let error = (1.0 - answer.value() / distance.value()).abs();
                    if error < 0.03 {
                        *data.score += 3;
                    } else if error < 0.05 {
                        *data.score += 2;
                    } else if error < 0.1 {
                        *data.score += 1;
                    }
                    *data.possible_score += 3;
                }
            }
            Err(_) => {
                self.state.answer_review_text_heading = "You didn't guess".to_string();
                self.state.answer_review_text = format!("The real distance was {:.1}°.", distance);
            }
        };
        data.used_questions.push(data.current_question);
        *data.game_stage = GameStage::Checked;
    }
}

impl crate::game::game_handler::QuestionTrait for Question {
    fn render_window(&mut self, data: QuestionWindowData) -> Option<egui::InnerResponse<Option<()>>> {
        if *data.game_stage == GameStage::Guessing {
            self.render_question_window(data)
        } else if *data.game_stage == GameStage::Checked {
            self.render_answer_review_window(data)
        } else {
            None
        }
    }

    fn generic_to_next_part(&mut self, data: QuestionCheckingData) {
        match data.game_stage {
            GameStage::Guessing => {
                if !self.should_display_input() {
                    self.check_answer(data);
                }
            }
            GameStage::Checked => {
                *data.start_next_question = true;
            }
            GameStage::NotStartedYet | GameStage::NoMoreQuestions | GameStage::ScoredModeFinished => {}
        }
    }

    fn can_choose_as_next(&self, questions_settings: &super::Settings, _active_constellations: &mut HashMap<String, bool>) -> bool {
        questions_settings.angular_separation.show
    }

    fn reset(self: Box<Self>) -> Box<dyn game_handler::QuestionTrait> {
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

    fn start_question(&mut self, questions_settings: &questions::Settings, cellestial_sphere: &mut CellestialSphere, theme: &Theme) {
        self.state = Default::default();
        let (ra1, dec1) = self.point1;
        let (ra2, dec2) = self.point2;
        cellestial_sphere.game_markers.markers = vec![
            GameMarker::new(GameMarkerType::Task, ra1, dec1, 2.0, 5.0, false, false, &theme.game_visuals.game_markers_colours),
            GameMarker::new(GameMarkerType::Task, ra2, dec2, 2.0, 5.0, false, false, &theme.game_visuals.game_markers_colours),
        ];
        if questions_settings.angular_separation.rotate_to_midpoint {
            let end_1 = sg_geometry::get_point_vector(ra1, dec1, &nalgebra::Matrix3::<f32>::identity());
            let end_2 = sg_geometry::get_point_vector(ra2, dec2, &nalgebra::Matrix3::<f32>::identity());
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

    fn clone_box(&self) -> Box<dyn game_handler::QuestionTrait> {
        Box::new(self.clone())
    }
}
