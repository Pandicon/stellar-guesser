use crate::action::Action;
use crate::enums::GameStage;
use crate::game::game_handler::{QuestionCheckingData, QuestionTrait, QuestionWindowData};
use crate::game::{game_handler, questions};
use crate::rendering::themes::Theme;
use crate::sky::markers::game_markers;
use angle::Deg;
use eframe::egui;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy)]
pub struct SmallSettings {
    pub rotate_to_point: bool,
    pub replay_incorrect: bool,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Settings {
    pub rotate_to_point: bool,
    pub limit_to_toggled_constellations: bool,
    pub magnitude_cutoff: f32,
    pub replay_incorrect: bool,
    pub show: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            rotate_to_point: true,
            limit_to_toggled_constellations: false,
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

    pub small_settings: SmallSettings,
}

impl Question {
    pub fn activate(&self) -> ActiveQuestion {
        ActiveQuestion {
            data: self.clone(),
            state: Default::default(),
        }
    }
}

#[derive(Clone)]
pub struct ActiveQuestion {
    pub data: Question,
    pub state: State,
}

impl ActiveQuestion {
    fn render_question_window(&mut self, data: QuestionWindowData, actions: &mut Vec<Action>) -> Option<egui::InnerResponse<Option<()>>> {
        egui::Window::new("Question").open(data.game_question_opened).show(data.ctx, |ui| {
            self.render_display_question(ui);
            if self.should_display_input() {
                let text_input_response = ui.text_edit_singleline(&mut self.state.answer);
                if *data.request_input_focus {
                    text_input_response.request_focus();
                    *data.request_input_focus = false;
                }
            }
            if ui.button("Check").clicked() {
                self.check_answer(
                    QuestionCheckingData {
                        sky: data.sky,
                        theme: data.theme,
                        game_stage: data.game_stage,
                        is_scored_mode: data.is_scored_mode,
                        current_question: data.current_question,
                        used_questions: data.used_questions,
                        add_marker_on_click: data.add_marker_on_click,
                        questions_settings: data.questions_settings,
                        question_number: data.question_number,
                    },
                    actions,
                );
            }
            ui.label(data.question_number_text);
        })
    }

    fn render_answer_review_window(&self, data: QuestionWindowData, actions: &mut Vec<Action>) -> Option<egui::InnerResponse<Option<()>>> {
        egui::Window::new("Question").open(data.game_question_opened).show(data.ctx, |ui| {
            if !self.state.answer_review_text_heading.is_empty() {
                ui.heading(&self.state.answer_review_text_heading);
            }
            ui.label(&self.state.answer_review_text);
            if ui.button("Next").clicked() {
                actions.push(Action::SwitchToNextPart);
            }
            ui.label(data.question_number_text);
        })
    }
    fn check_answer(&mut self, data: QuestionCheckingData, actions: &mut Vec<Action>) {
        match self.state.answer.parse::<f32>() {
            Ok(answer) => {
                let error = (self.data.mag - answer).abs();
                self.state.answer_review_text_heading = format!("You were {error:.1} mag away!");

                self.state.answer_review_text = format!("The magnitude was {:.1}.", self.data.mag);

                if data.is_scored_mode {
                    if error < 0.3 {
                        actions.push(Action::ChangeScore(3));
                    } else if error < 0.7 {
                        actions.push(Action::ChangeScore(2));
                    } else if error < 1.5 {
                        actions.push(Action::ChangeScore(1));
                    }
                    actions.push(Action::ChangePossibleScore(3));
                }
            }
            Err(_) => {
                self.state.answer_review_text_heading = "You didn't guess".to_string();
                self.state.answer_review_text = format!("The magnitude was {:.1}.", self.data.mag);
            }
        };
        data.used_questions.push(data.current_question);
        *data.game_stage = GameStage::Checked;
    }
}

impl crate::game::game_handler::QuestionTrait for ActiveQuestion {
    fn render_window(&mut self, data: QuestionWindowData, actions: &mut Vec<Action>) -> Option<egui::InnerResponse<Option<()>>> {
        if *data.game_stage == GameStage::Guessing {
            self.render_question_window(data, actions)
        } else if *data.game_stage == GameStage::Checked {
            self.render_answer_review_window(data, actions)
        } else {
            None
        }
    }

    fn generic_to_next_part(&mut self, data: QuestionCheckingData, actions: &mut Vec<Action>) {
        match data.game_stage {
            GameStage::Guessing => {
                if !self.should_display_input() {
                    self.check_answer(data, actions);
                }
            }
            GameStage::Checked => {
                actions.push(Action::SwitchToNextQuestion);
            }
            GameStage::NotStartedYet | GameStage::NoMoreQuestions | GameStage::ScoredModeFinished => {}
        }
    }

    fn reset(self: Box<Self>) -> Box<dyn game_handler::QuestionTrait> {
        Box::new(Self {
            data: Question {
                ra: self.data.ra,
                dec: self.data.dec,
                mag: self.data.mag,

                small_settings: self.data.small_settings,
            },
            state: State::default(),
        })
    }

    fn show_tolerance_marker(&self) -> bool {
        false
    }

    fn show_circle_marker(&self) -> bool {
        true
    }

    fn get_question_distance_tolerance(&self) -> Deg<f32> {
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

    fn start_question(&mut self, theme: &Theme, actions: &mut Vec<Action>) {
        self.state = Default::default();
        actions.push(Action::SetGameMarkers(vec![game_markers::GameMarker::new(
            game_markers::GameMarkerType::Task,
            self.data.ra,
            self.data.dec,
            2.0,
            5.0,
            true,
            false,
            &theme.game_visuals.game_markers_colours,
        )]));
        if self.data.small_settings.rotate_to_point {
            let final_vector = sg_geometry::get_point_vector(self.data.ra, self.data.dec, &nalgebra::Matrix3::<f32>::identity());
            actions.push(Action::CameraLookAt(final_vector));
        }
    }

    fn render_display_question(&self, ui: &mut egui::Ui) {
        ui.heading("What is the magnitude of this object?");
    }

    fn clone_box(&self) -> Box<dyn game_handler::QuestionTrait> {
        Box::new(self.clone())
    }
}

pub fn generate_questions(objects: &[&crate::game::QuestionObject], small_settings: SmallSettings) -> Vec<questions::Question> {
    let mut questions: Vec<questions::Question> = Vec::with_capacity(objects.len());
    for object in objects {
        if let Some(mag) = object.mag {
            questions.push(questions::Question::GuessTheMagnitude(Question {
                ra: object.ra,
                dec: object.dec,
                mag,
                small_settings,
            }));
        }
    }
    questions
}
