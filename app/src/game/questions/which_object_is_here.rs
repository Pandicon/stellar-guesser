use crate::action::Action;
use crate::enums::GameStage;
use crate::game::questions;
use crate::rendering::themes::Theme;
use crate::sky::markers::game_markers;
use angle::Deg;
use eframe::egui;
use rand::Rng;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy)]
#[serde(default)]
pub struct SmallSettings {
    pub rotate_to_point: bool,
    pub replay_incorrect: bool,
    pub accept_messier: bool,
    pub accept_caldwell: bool,
    pub accept_ngc: bool,
    pub accept_ic: bool,
    pub accept_hip: bool,
    pub accept_hd: bool,
    pub accept_proper: bool,
    pub accept_bayer: bool,
    pub accept_flamsteed: bool,
}

impl Default for SmallSettings {
    fn default() -> Self {
        Self {
            rotate_to_point: true,
            replay_incorrect: true,
            accept_messier: true,
            accept_caldwell: true,
            accept_ngc: true,
            accept_ic: true,
            accept_hip: true,
            accept_hd: true,
            accept_proper: true,
            accept_bayer: true,
            accept_flamsteed: true,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Settings {
    pub rotate_to_point: bool,
    pub limit_to_toggled_constellations: bool,
    pub show_messiers: bool,
    pub show_caldwells: bool,
    pub show_ngcs: bool,
    pub show_ics: bool,
    pub show_bayer: bool,
    pub show_starnames: bool,
    pub magnitude_cutoff: f32,
    pub correctness_threshold: f32,
    pub replay_incorrect: bool,
    pub show: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            rotate_to_point: true,
            limit_to_toggled_constellations: true,
            show_messiers: true,
            show_caldwells: true,
            show_ngcs: true,
            show_ics: true,
            show_bayer: true,
            show_starnames: true,
            magnitude_cutoff: 6.0,
            correctness_threshold: 0.2,
            replay_incorrect: true,
            show: true,
        }
    }
}

#[derive(Clone, Default)]
pub struct State {
    answer: String,
    answer_image: Option<crate::structs::image_info::ImageInfo>,

    answer_review_text_heading: String,
    answer_review_text: String,
}

#[derive(Clone)]
pub struct Question {
    pub small_settings: SmallSettings,
    pub possible_names: Vec<String>,
    pub ra: angle::Deg<f32>,
    pub dec: angle::Deg<f32>,
    pub is_messier: bool,
    pub is_caldwell: bool,
    pub is_ngc: bool,
    pub is_ic: bool,
    pub is_bayer: bool,
    pub is_starname: bool,
    pub magnitude: Option<f32>,
    pub object_type: String,
    pub constellation_abbreviation: String,
    pub images: Vec<crate::structs::image_info::ImageInfo>,
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
            if let Some(image) = &self.state.answer_image {
                ui.add(egui::Image::new(&image.path).max_width(600.0));
                if let Some(image_source) = &image.source {
                    ui.hyperlink_to("Image source", image_source);
                }
            }
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

    pub fn check_answer(&mut self, question_id: usize, actions: &mut Vec<Action>) {
        if !self.data.images.is_empty() {
            self.state.answer_image = Some(self.data.images[rand::thread_rng().gen_range(0..self.data.images.len())].clone());
        }
        let possible_names_edited = self.data.possible_names.iter().map(|name| name.replace(' ', "").to_lowercase()).collect::<Vec<String>>();
        let correct = possible_names_edited.contains(&self.state.answer.replace(' ', "").to_lowercase());
        self.state.answer_review_text_heading = format!(
            "{}orrect!",
            if correct {
                actions.push(Action::ChangeScore(1));
                "C"
            } else {
                "Inc"
            }
        );
        self.state.answer_review_text = format!(
            "Your answer was: {}\nPossible answers: {}\nObject type: {}",
            self.state.answer,
            self.data.possible_names.join(", "),
            self.data.object_type
        );
        actions.push(Action::ChangePossibleScore(1));
        if !self.data.small_settings.replay_incorrect || correct {
            actions.push(Action::MarkQuestionAsUsed(question_id));
        } else {
            actions.push(Action::IncrementRepeatedQuestionCounter);
        }
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

    pub fn generic_to_next_part(&mut self, question_id: usize, game_stage: &GameStage, actions: &mut Vec<Action>) {
        match game_stage {
            GameStage::Guessing => {
                self.check_answer(question_id, actions);
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
        self.data.is_bayer || self.data.is_starname
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
        let new_markers = if self.data.is_bayer || self.data.is_starname {
            vec![game_markers::GameMarker::new(
                game_markers::GameMarkerType::Task,
                self.data.ra,
                self.data.dec,
                2.0,
                5.0,
                true,
                false,
                &theme.game_visuals.game_markers_colours,
            )]
        } else {
            vec![game_markers::GameMarker::new(
                game_markers::GameMarkerType::Task,
                self.data.ra,
                self.data.dec,
                2.0,
                5.0,
                false,
                false,
                &theme.game_visuals.game_markers_colours,
            )]
        };
        actions.push(Action::SetGameMarkers(new_markers));
        if self.data.small_settings.rotate_to_point {
            let final_vector = sg_geometry::get_point_vector(self.data.ra, self.data.dec, &nalgebra::Matrix3::<f32>::identity());
            actions.push(Action::CameraLookAt(final_vector));
        }
    }

    fn render_display_question(&self, ui: &mut egui::Ui) {
        let mut accepted = Vec::new();
        if self.data.small_settings.accept_bayer {
            accepted.push("Bayer");
        }
        if self.data.small_settings.accept_caldwell {
            accepted.push("Caldwell");
        }
        if self.data.small_settings.accept_flamsteed {
            accepted.push("Flamsteed");
        }
        if self.data.small_settings.accept_hd {
            accepted.push("HD");
        }
        if self.data.small_settings.accept_hip {
            accepted.push("HIP");
        }
        if self.data.small_settings.accept_ic {
            accepted.push("IC");
        }
        if self.data.small_settings.accept_messier {
            accepted.push("Messier");
        }
        if self.data.small_settings.accept_ngc {
            accepted.push("NGC");
        }
        if self.data.small_settings.accept_proper {
            accepted.push("Proper name");
        }
        ui.heading("What is this object?");
        ui.label(format!("Accepted names: {}", accepted.join(", ")));
    }
}

pub fn generate_questions(objects: &[&crate::game::QuestionObject], small_settings: SmallSettings) -> Vec<questions::Question> {
    let mut questions: Vec<questions::Question> = Vec::with_capacity(objects.len());
    for object in objects {
        let mut possible_names = Vec::new();
        if small_settings.accept_bayer {
            if let Some(designation) = &object.bayer_designation_raw {
                let names = crate::rendering::caspr::generate_name_combinations(designation, crate::rendering::caspr::SpecificName::None);
                possible_names.extend(names);
            }
        }
        if small_settings.accept_caldwell {
            if let Some(designation) = object.caldwell_number {
                possible_names.push(format!("C{designation}"));
            }
        }
        if small_settings.accept_flamsteed {
            if let Some(designation) = &object.flamsteed_designation_raw {
                let names = crate::rendering::caspr::generate_name_combinations(designation, crate::rendering::caspr::SpecificName::None);
                possible_names.extend(names);
            }
        }
        if small_settings.accept_hd {
            if let Some(designation) = &object.hd_number {
                possible_names.push(format!("HD{designation}"));
            }
        }
        if small_settings.accept_hip {
            if let Some(designation) = &object.hipparcos_number {
                possible_names.push(format!("HIP{designation}"));
            }
        }
        if small_settings.accept_ic {
            if let Some(designation) = &object.ic_number {
                possible_names.push(format!("IC{designation}"));
            }
        }
        if small_settings.accept_messier {
            if let Some(designation) = &object.messier_number {
                possible_names.push(format!("M{designation}"));
            }
        }
        if small_settings.accept_ngc {
            if let Some(designation) = &object.ngc_number {
                possible_names.push(format!("NGC{designation}"));
            }
        }
        if small_settings.accept_proper {
            for name in &object.proper_names_raw {
                let names = crate::rendering::caspr::generate_name_combinations(name, crate::rendering::caspr::SpecificName::None);
                possible_names.extend(names);
            }
        }
        if !possible_names.is_empty() {
            questions.push(questions::Question::WhatIsThisObject(Question {
                small_settings,
                possible_names,
                ra: object.ra,
                dec: object.dec,
                is_messier: object.messier_number.is_some(),
                is_caldwell: object.caldwell_number.is_some(),
                is_ngc: object.ngc_number.is_some(),
                is_ic: object.ic_number.is_some(),
                is_bayer: object.bayer_designation_full.is_some(),
                images: object.images.clone(),
                is_starname: matches!(object.object_type, crate::game::ObjectType::Star(_)),
                magnitude: object.mag,
                object_type: match &object.object_type {
                    crate::game::ObjectType::Star(star_type) => star_type.display_name(),
                    crate::game::ObjectType::Deepsky(deepsky_type) => deepsky_type.display_name(),
                },
                constellation_abbreviation: object.constellations_abbreviations.first().cloned().unwrap_or(String::from("Unknown")),
            }));
        }
    }
    questions
}
