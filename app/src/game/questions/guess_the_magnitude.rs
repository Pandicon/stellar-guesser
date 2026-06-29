use crate::action::Action;
use crate::enums::GameStage;
use crate::game::questions;
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
    fn render_question_window(
        &mut self,
        ctx: &eframe::egui::Context,
        game_question_opened: bool,
        request_input_focus: bool,
        question_number_text: &str,
        actions: &mut Vec<Action>,
    ) -> Option<egui::InnerResponse<Option<()>>> {
        let mut is_window_open = game_question_opened;
        let response = egui::Window::new("Question").open(&mut is_window_open).show(ctx, |ui| {
            self.render_display_question(ui);
            if self.should_display_input() {
                let text_input_response = ui.text_edit_singleline(&mut self.state.answer);
                if request_input_focus {
                    text_input_response.request_focus();
                    actions.push(Action::SetRequestInputFocus(false));
                }
            }
            if ui.button("Check").clicked() {
                actions.push(Action::CheckAnswer);
            }
            ui.label(question_number_text);
        });
        if is_window_open != game_question_opened {
            actions.push(Action::ToggleQuestionWindow(is_window_open));
        }
        response
    }

    fn render_answer_review_window(&self, ctx: &eframe::egui::Context, game_question_opened: bool, question_number_text: &str, actions: &mut Vec<Action>) -> Option<egui::InnerResponse<Option<()>>> {
        let mut is_window_open = game_question_opened;
        let response = egui::Window::new("Question").open(&mut is_window_open).show(ctx, |ui| {
            if !self.state.answer_review_text_heading.is_empty() {
                ui.heading(&self.state.answer_review_text_heading);
            }
            ui.label(&self.state.answer_review_text);
            if ui.button("Next").clicked() {
                actions.push(Action::SwitchToNextPart);
            }
            ui.label(question_number_text);
        });
        if is_window_open != game_question_opened {
            actions.push(Action::ToggleQuestionWindow(is_window_open));
        }
        response
    }

    pub fn check_answer(&mut self, is_scored_mode: bool, question_id: usize, actions: &mut Vec<Action>) {
        match self.state.answer.parse::<f32>() {
            Ok(answer) => {
                let error = (self.data.mag - answer).abs();
                self.state.answer_review_text_heading = format!("You were {error:.1} mag away!");

                self.state.answer_review_text = format!("The magnitude was {:.1}.", self.data.mag);

                if is_scored_mode {
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
        actions.push(Action::MarkQuestionAsUsed(question_id));
        actions.push(Action::SetGameStage(GameStage::Checked));
    }
}

impl ActiveQuestion {
    pub fn render_window(
        &mut self,
        ctx: &eframe::egui::Context,
        game_stage: GameStage,
        game_question_opened: bool,
        request_input_focus: bool,
        question_number_text: &str,
        actions: &mut Vec<Action>,
    ) -> Option<egui::InnerResponse<Option<()>>> {
        if game_stage == GameStage::Guessing {
            self.render_question_window(ctx, game_question_opened, request_input_focus, question_number_text, actions)
        } else if game_stage == GameStage::Checked {
            self.render_answer_review_window(ctx, game_question_opened, question_number_text, actions)
        } else {
            None
        }
    }

    pub fn generic_to_next_part(&mut self, is_scored_mode: bool, question_id: usize, game_stage: &GameStage, actions: &mut Vec<Action>) {
        match game_stage {
            GameStage::Guessing => {
                self.check_answer(is_scored_mode, question_id, actions);
            }
            GameStage::Checked => {
                actions.push(Action::SwitchToNextQuestion);
                actions.push(Action::RemoveGameMarkers);
            }
            GameStage::NotStartedYet | GameStage::NoMoreQuestions | GameStage::ScoredModeFinished => {}
        }
    }

    pub fn show_tolerance_marker(&self) -> bool {
        false
    }

    pub fn show_circle_marker(&self) -> bool {
        true
    }

    pub fn get_question_distance_tolerance(&self) -> Deg<f32> {
        angle::Deg(0.0)
    }

    pub fn allow_multiple_player_markers(&self) -> bool {
        false
    }

    pub fn add_marker_on_click(&self) -> bool {
        false
    }

    pub fn should_display_input(&self) -> bool {
        true
    }

    pub fn start_question(&mut self, theme: &Theme, actions: &mut Vec<Action>) {
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
