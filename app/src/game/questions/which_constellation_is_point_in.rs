use crate::action::Action;
use crate::enums::GameStage;
use crate::game::game_handler::{QuestionCheckingData, QuestionWindowData};
use crate::game::questions;
use crate::rendering::themes::Theme;
use crate::sky;
use crate::sky::markers::game_markers;
use angle::{Angle, Deg};
use eframe::egui;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy)]
pub struct SmallSettings {
    pub rotate_to_point: bool,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Settings {
    pub show: bool,
    pub rotate_to_point: bool,
    pub limit_to_toggled_constellations: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            show: true,
            rotate_to_point: true,
            limit_to_toggled_constellations: true,
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
                    actions.push(Action::SetRequestInputFocus(false));
                }
            }
            if ui.button("Check").clicked() {
                self.check_answer(data.current_question, data.sky, actions);
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

    fn check_answer(&mut self, question_id: usize, sky: &sky::Sky, actions: &mut Vec<Action>) {
        let possible_abbrevs = sky.determine_constellation((self.data.ra.to_rad(), self.data.dec.to_rad()));
        let mut possible_constellation_names = Vec::new();
        for abbrev in possible_abbrevs {
            if let Some(constellation) = sky.constellations.get(&abbrev) {
                possible_constellation_names.extend(constellation.possible_names.iter().map(|name| name.replace(' ', "").to_lowercase()));
            };
        }
        let correct = possible_constellation_names.contains(&self.state.answer.replace(' ', "").to_lowercase());
        self.state.answer_review_text_heading = format!(
            "{}orrect!",
            if correct {
                actions.push(Action::ChangeScore(1));
                "C"
            } else {
                "Inc"
            }
        );
        actions.push(Action::ChangePossibleScore(1));
        self.state.answer_review_text = format!("Your answer was: {}\nThe right answers were: {}", self.state.answer, possible_constellation_names.join(", "));
        actions.push(Action::MarkQuestionAsUsed(question_id));
        actions.push(Action::SetGameStage(GameStage::Checked));
    }
}

impl ActiveQuestion {
    pub fn render_window(&mut self, data: QuestionWindowData, actions: &mut Vec<Action>) -> Option<egui::InnerResponse<Option<()>>> {
        if *data.game_stage == GameStage::Guessing {
            self.render_question_window(data, actions)
        } else if *data.game_stage == GameStage::Checked {
            self.render_answer_review_window(data, actions)
        } else {
            None
        }
    }

    pub fn generic_to_next_part(&mut self, data: QuestionCheckingData, actions: &mut Vec<Action>) {
        match data.game_stage {
            GameStage::Guessing => {
                if !self.should_display_input() {
                    self.check_answer(data.current_question, data.sky, actions);
                }
            }
            GameStage::Checked => {
                actions.push(Action::SwitchToNextQuestion);
            }
            GameStage::NotStartedYet | GameStage::NoMoreQuestions | GameStage::ScoredModeFinished => {}
        }
    }

    pub fn show_tolerance_marker(&self) -> bool {
        false
    }

    pub fn show_circle_marker(&self) -> bool {
        false
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
            false,
            false,
            &theme.game_visuals.game_markers_colours,
        )]));
        if self.data.small_settings.rotate_to_point {
            let final_vector = sg_geometry::get_point_vector(self.data.ra, self.data.dec, &nalgebra::Matrix3::<f32>::identity());
            actions.push(Action::CameraLookAt(final_vector));
        }
    }

    fn render_display_question(&self, ui: &mut egui::Ui) {
        ui.heading("What constellation does this point lie in?");
    }
}

pub fn generate_questions(objects: &[&crate::game::QuestionObject], small_settings: SmallSettings) -> Vec<questions::Question> {
    let mut questions: Vec<questions::Question> = Vec::with_capacity(objects.len());
    for object in objects {
        questions.push(questions::Question::WhichConstellationIsThisPointIn(Question {
            ra: object.ra,
            dec: object.dec,
            small_settings,
        }));
    }
    questions
}
