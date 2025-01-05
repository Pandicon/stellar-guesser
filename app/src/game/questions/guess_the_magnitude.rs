use crate::enums::GameStage;
use crate::game::game_handler::{GameHandler, QuestionCheckingData, QuestionTrait, QuestionWindowData};
use crate::game::{game_handler, questions};
use crate::renderer::CellestialSphere;
use crate::rendering::caspr::markers::game_markers::{GameMarker, GameMarkerType};
use crate::rendering::themes::Theme;
use angle::Deg;
use eframe::egui;
use std::collections::HashMap;

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

    answer_review_text_heading: String,
    answer_review_text: String,
}

#[derive(Clone)]
pub struct Question {
    pub ra: angle::Deg<f32>,
    pub dec: angle::Deg<f32>,
    pub mag: f32,

    pub state: State,
}

impl Question {
    fn render_question_window(&mut self, data: QuestionWindowData) -> Option<egui::InnerResponse<Option<()>>> {
        egui::Window::new("Question").open(data.game_question_opened).show(data.ctx, |ui| {
            ui.heading(self.get_display_question());
            if self.should_display_input() {
                let text_input_response = ui.text_edit_singleline(&mut self.state.answer);
                if *data.request_input_focus {
                    text_input_response.request_focus();
                    *data.request_input_focus = false;
                }
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
        match self.state.answer.parse::<f32>() {
            Ok(answer) => {
                let error = (self.mag - answer).abs();
                self.state.answer_review_text_heading = format!("You were {:.1} mag away!", error);

                self.state.answer_review_text = format!("The magnitude was {:.1}.", self.mag);

                if data.is_scored_mode {
                    if error < 0.3 {
                        *data.score += 3;
                    } else if error < 0.7 {
                        *data.score += 2;
                    } else if error < 1.5 {
                        *data.score += 1;
                    }
                    *data.possible_score += 3;
                }
            }
            Err(_) => {
                self.state.answer_review_text_heading = "You didn't guess".to_string();
                self.state.answer_review_text = format!("The magnitude was {:.1}.", self.mag);
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
        questions_settings.guess_the_magnitude.show && self.mag < questions_settings.guess_the_magnitude.magnitude_cutoff
    }

    fn reset(self: Box<Self>) -> Box<dyn game_handler::QuestionTrait> {
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

    fn start_question(&mut self, questions_settings: &questions::Settings, cellestial_sphere: &mut CellestialSphere, theme: &Theme) {
        self.state = Default::default();
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
        if questions_settings.guess_the_magnitude.rotate_to_point {
            let final_vector = sg_geometry::get_point_vector(self.ra, self.dec, &nalgebra::Matrix3::<f32>::identity());
            cellestial_sphere.look_at_point(&final_vector);
            cellestial_sphere.init_renderers();
        }
    }

    fn get_display_question(&self) -> String {
        String::from("What is the magnitude of this star?")
    }

    fn clone_box(&self) -> Box<dyn game_handler::QuestionTrait> {
        Box::new(self.clone())
    }
}
