use crate::action::Action;
use crate::enums::GameStage;
use crate::game::game_handler::{QuestionCheckingData, QuestionWindowData};
use crate::game::questions;
use crate::rendering::themes::Theme;
use crate::sky::markers::game_markers;
use angle::{Angle, Deg};
use eframe::egui;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy)]
pub struct SmallSettings {
    pub rotate_to_midpoint: bool,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Settings {
    pub show: bool,
    pub rotate_to_midpoint: bool,
    pub limit_to_toggled_constellations: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            show: true,
            rotate_to_midpoint: true,
            limit_to_toggled_constellations: false,
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
    /// (ra, dec)
    pub point1: (angle::Deg<f32>, angle::Deg<f32>),
    /// (ra, dec)
    pub point2: (angle::Deg<f32>, angle::Deg<f32>),

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
        let mut is_window_open = *data.game_question_opened;
        let response = egui::Window::new("Question").open(&mut is_window_open).show(data.ctx, |ui| {
            self.render_display_question(ui);
            if self.should_display_input() {
                let text_input_response = ui.text_edit_singleline(&mut self.state.answer);
                if *data.request_input_focus {
                    text_input_response.request_focus();
                    actions.push(Action::SetRequestInputFocus(false));
                }
            }
            if ui.button("Check").clicked() {
                self.check_answer(data.is_scored_mode, data.current_question, actions);
            }
            ui.label(data.question_number_text);
        });
        if is_window_open != *data.game_question_opened {
            actions.push(Action::ToggleQuestionWindow(is_window_open));
        }
        response
    }

    fn render_answer_review_window(&self, data: QuestionWindowData, actions: &mut Vec<Action>) -> Option<egui::InnerResponse<Option<()>>> {
        let mut is_window_open = *data.game_question_opened;
        let response = egui::Window::new("Question").open(&mut is_window_open).show(data.ctx, |ui| {
            if !self.state.answer_review_text_heading.is_empty() {
                ui.heading(&self.state.answer_review_text_heading);
            }
            ui.label(&self.state.answer_review_text);
            if ui.button("Next").clicked() {
                actions.push(Action::SwitchToNextPart);
            }
            ui.label(data.question_number_text);
        });
        if is_window_open != *data.game_question_opened {
            actions.push(Action::ToggleQuestionWindow(is_window_open));
        }
        response
    }

    fn check_answer(&mut self, is_scored_mode: bool, question_id: usize, actions: &mut Vec<Action>) {
        let (ra1, dec1) = self.data.point1;
        let (ra2, dec2) = self.data.point2;
        let distance = sg_geometry::angular_distance((ra1.to_rad(), dec1.to_rad()), (ra2.to_rad(), dec2.to_rad())).to_deg();
        match self.state.answer.parse::<f32>() {
            Ok(answer) => {
                let answer = angle::Deg(answer);
                self.state.answer_review_text_heading = format!("You were {:.1} degrees away!", (distance - answer).value());
                let error_percent = 1.0 - answer.value() / distance.value();
                self.state.answer_review_text = format!("The real distance was {:.1}°. Your error is equal to {:.1}% of the distance.", distance.value(), error_percent * 100.0);
                if is_scored_mode {
                    let error = (1.0 - answer.value() / distance.value()).abs();
                    if error < 0.03 {
                        actions.push(Action::ChangeScore(3));
                    } else if error < 0.05 {
                        actions.push(Action::ChangeScore(2));
                    } else if error < 0.1 {
                        actions.push(Action::ChangeScore(1));
                    }
                    actions.push(Action::ChangePossibleScore(3));
                }
            }
            Err(_) => {
                self.state.answer_review_text_heading = "You didn't guess".to_string();
                self.state.answer_review_text = format!("The real distance was {distance:.1}°.");
            }
        };
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
                    self.check_answer(data.is_scored_mode, data.current_question, actions);
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
        let (ra1, dec1) = self.data.point1;
        let (ra2, dec2) = self.data.point2;
        actions.push(Action::SetGameMarkers(vec![
            game_markers::GameMarker::new(game_markers::GameMarkerType::Task, ra1, dec1, 2.0, 5.0, false, false, &theme.game_visuals.game_markers_colours),
            game_markers::GameMarker::new(game_markers::GameMarkerType::Task, ra2, dec2, 2.0, 5.0, false, false, &theme.game_visuals.game_markers_colours),
        ]));
        if self.data.small_settings.rotate_to_midpoint {
            let end_1 = sg_geometry::get_point_vector(ra1, dec1, &nalgebra::Matrix3::<f32>::identity());
            let end_2 = sg_geometry::get_point_vector(ra2, dec2, &nalgebra::Matrix3::<f32>::identity());
            if (end_1 + end_2).magnitude_squared() > 10e-4 {
                let final_vector = (end_1 + end_2).normalize();
                actions.push(Action::CameraLookAt(final_vector));
            }
        }
    }

    fn render_display_question(&self, ui: &mut egui::Ui) {
        ui.heading("What is the angular distance between these markers?");
    }
}

pub fn generate_questions(objects: &[&crate::game::QuestionObject], small_settings: SmallSettings) -> Vec<questions::Question> {
    let mut questions: Vec<questions::Question> = Vec::with_capacity(objects.len() / 2);
    for i in (0..objects.len()).step_by(2) {
        if i + 1 >= objects.len() {
            break;
        }
        questions.push(questions::Question::AngularSeparation(Question {
            point1: (objects[i].ra, objects[i].dec),
            point2: (objects[i + 1].ra, objects[i + 1].dec),
            small_settings,
        }));
    }
    questions
}
