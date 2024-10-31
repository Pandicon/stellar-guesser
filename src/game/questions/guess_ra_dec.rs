use crate::enums::GameStage;
use crate::game::game_handler;
use crate::game::game_handler::{GameHandler, QuestionCheckingData, QuestionTrait, QuestionWindowData};
use crate::geometry;
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

    answer_review_text_heading: String,
    answer_review_text: String,
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
                });
            }
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
        match self.state.answer.parse::<f32>() {
            Ok(answer_hours) => {
                let answer_deg = angle::Deg(answer_hours / 24.0 * 360.0);
                let error_deg = (self.ra - answer_deg).abs();
                self.state.answer_review_text_heading = format!("You were {:.1}h away!", error_deg.value() / 360.0 * 24.0);

                self.state.answer_review_text = format!("The real right ascension was {:.1}h", self.ra.value() / 360.0 * 24.0);

                if data.is_scored_mode {
                    if error_deg < angle::Deg(3.0) {
                        *data.score += 3;
                    } else if error_deg < angle::Deg(5.0) {
                        *data.score += 2;
                    } else if error_deg < angle::Deg(10.0) {
                        *data.score += 1;
                    }
                    *data.possible_score += 3;
                }
            }
            Err(_) => {
                self.state.answer_review_text_heading = "You didn't guess".to_string();
                self.state.answer_review_text = format!("The real right ascension was {:.1}h.", self.ra.value() / 360.0 * 24.0);
            }
        };
        data.used_questions.push(data.current_question);
        *data.game_stage = GameStage::Checked;
    }
}

impl crate::game::game_handler::QuestionTrait for RaQuestion {
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
            GameStage::Checked => {}
            GameStage::NotStartedYet | GameStage::NoMoreQuestions | GameStage::ScoredModeFinished => {}
        }
    }

    fn can_choose_as_next(&self, questions_settings: &super::Settings, _active_constellations: &mut HashMap<String, bool>) -> bool {
        questions_settings.guess_rad_dec.show
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

    fn start_question(&self, game_handler: &GameHandler, cellestial_sphere: &mut CellestialSphere, theme: &Theme) {
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

    fn clone_box(&self) -> Box<dyn game_handler::QuestionTrait> {
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
                });
            }
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
        match self.state.answer.parse::<f32>() {
            Ok(answer) => {
                let answer_deg = angle::Deg(answer);
                let error = (self.dec - answer_deg).abs();
                self.state.answer_review_text_heading = format!("You were {:.1}° away!", error.value());

                self.state.answer_review_text = format!("The declination was {:.1}°", self.dec.value());

                if data.is_scored_mode {
                    if error < angle::Deg(3.0) {
                        *data.score += 3;
                    } else if error < angle::Deg(5.0) {
                        *data.score += 2;
                    } else if error < angle::Deg(10.0) {
                        *data.score += 1;
                    }
                    *data.possible_score += 3;
                }
            }
            Err(_) => {
                self.state.answer_review_text_heading = "You didn't guess".to_string();
                self.state.answer_review_text = format!("The declination was {:.1}°.", self.dec);
            }
        };
        data.used_questions.push(data.current_question);
        *data.game_stage = GameStage::Checked;
    }
}

impl crate::game::game_handler::QuestionTrait for DecQuestion {
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
            GameStage::Checked => {}
            GameStage::NotStartedYet | GameStage::NoMoreQuestions | GameStage::ScoredModeFinished => {}
        }
    }

    fn can_choose_as_next(&self, questions_settings: &super::Settings, _active_constellations: &mut HashMap<String, bool>) -> bool {
        questions_settings.guess_rad_dec.show
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

    fn start_question(&self, game_handler: &GameHandler, cellestial_sphere: &mut CellestialSphere, theme: &Theme) {
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

    fn clone_box(&self) -> Box<dyn game_handler::QuestionTrait> {
        Box::new(self.clone())
    }
}
