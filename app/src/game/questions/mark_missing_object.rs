use crate::action::Action;
use crate::enums::{GameStage, RendererCategory};
use crate::game::game_handler::{GameHandler, QuestionCheckingData, QuestionWindowData};
use crate::game::questions;
use crate::rendering::themes::Theme;
use crate::sky::markers::game_markers;
use angle::{Angle, Deg};
use eframe::egui;
use rand::Rng;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy)]
#[serde(default)]
pub struct SmallSettings {
    pub correctness_threshold: angle::Deg<f32>,
    pub rotate_to_answer: bool,
    pub replay_incorrect: bool,
}

impl Default for SmallSettings {
    fn default() -> Self {
        Self {
            correctness_threshold: angle::Deg(1.0),
            rotate_to_answer: true,
            replay_incorrect: true,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Settings {
    pub rotate_to_correct_point: bool,
    pub limit_to_toggled_constellations: bool,
    pub show_messiers: bool,
    pub show_caldwells: bool,
    pub show_ngcs: bool,
    pub show_ics: bool,
    pub show_bayer: bool,
    pub show_starnames: bool,
    pub magnitude_cutoff: f32,
    pub correctness_threshold: angle::Deg<f32>,
    pub replay_incorrect: bool,
    pub show: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            rotate_to_correct_point: true,
            limit_to_toggled_constellations: true,
            show_messiers: true,
            show_caldwells: true,
            show_ngcs: true,
            show_ics: true,
            show_bayer: true,
            show_starnames: true,
            magnitude_cutoff: 6.0,
            correctness_threshold: angle::Deg(0.2),
            replay_incorrect: true,
            show: true,
        }
    }
}

#[derive(Clone, Default)]
pub struct State {
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
    pub object_id: u64,
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
            if ui.button("Check").clicked() {
                self.check_answer(data.is_scored_mode, data.current_question, &data.sky.game_markers.markers, data.theme, actions);
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
            if let Some(image) = &self.state.answer_image {
                ui.add(egui::Image::new(&image.path).max_width(600.0));
                if let Some(image_source) = &image.source {
                    ui.hyperlink_to("Image source", image_source);
                }
            }
            if ui.button("Next").clicked() {
                actions.push(Action::SwitchToNextPart);
            }
            ui.label(data.question_number_text);
        })
    }

    fn check_answer(&mut self, is_scored_mode: bool, question_id: usize, markers: &[game_markers::GameMarker], theme: &Theme, actions: &mut Vec<Action>) {
        actions.push(Action::SetAddMarkerOnClick(false));
        let mut correct = false;
        if !self.data.images.is_empty() {
            self.state.answer_image = Some(self.data.images[rand::thread_rng().gen_range(0..self.data.images.len())].clone());
        }
        let (answer_dec_text, answer_ra_text, distance, answer_review_text_heading) = if !markers.is_empty() {
            let answer_dec = markers[0].dec;
            let answer_ra = markers[0].ra;
            let distance = sg_geometry::angular_distance((self.data.ra.to_rad(), self.data.dec.to_rad()), (answer_ra.to_rad(), answer_dec.to_rad())).to_deg();
            if is_scored_mode {
                let score_delta = GameHandler::evaluate_score(distance);
                actions.push(Action::ChangeScore(score_delta));
            }
            (
                answer_dec.value().to_string(),
                answer_ra.value().to_string(),
                distance.value().to_string(),
                if distance < self.data.small_settings.correctness_threshold {
                    correct = true;
                    String::from("Correct!")
                } else {
                    format!("You were {} degrees away from the missing object!", (distance.value() * 100.0).round() / 100.0)
                },
            )
        } else {
            (String::from("-"), String::from("-"), String::from("-"), "You didn't guess where the missing object is".to_string())
        };
        self.state.answer_review_text_heading = answer_review_text_heading;
        self.state.answer_review_text = format!(
            "Designations of the missing object: {}\nYour coordinates: [dec = {}°; ra = {}°]\nCorrect coordinates: [dec = {}°; ra = {}°]\nFully precise distance: {}°\nYou can see the correct place marked with a new {}.\nObject type: {}",
            self.data.possible_names.join(", "),
            answer_dec_text,
            answer_ra_text,
            self.data.dec.value(),
            self.data.ra.value(),
            distance,
            if self.data.is_bayer || self.data.is_starname { "circle" } else { "cross" },
            self.data.object_type
        );
        actions.push(Action::AddGameMarker(game_markers::GameMarker::new(
            game_markers::GameMarkerType::CorrectAnswer,
            self.data.ra,
            self.data.dec,
            2.0,
            5.0,
            self.data.is_bayer || self.data.is_starname,
            false,
            &theme.game_visuals.game_markers_colours,
        )));
        if !self.data.small_settings.replay_incorrect || correct {
            actions.push(Action::MarkQuestionAsUsed(question_id));
        } else {
            actions.push(Action::IncrementRepeatedQuestionCounter);
        }
        if self.data.small_settings.rotate_to_answer {
            let final_vector = sg_geometry::get_point_vector(self.data.ra, self.data.dec, &nalgebra::Matrix3::<f32>::identity());
            actions.push(Action::CameraLookAt(final_vector));
        } else {
            actions.push(Action::InitSingleRendererGroup(RendererCategory::Markers, String::from("game")));
        }
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
                self.check_answer(data.is_scored_mode, data.current_question, &data.sky.game_markers.markers, data.theme, actions);
            }
            GameStage::Checked => {
                actions.push(Action::SwitchToNextQuestion);
                actions.push(Action::EnableSingleRenderer(self.data.object_id));
            }
            GameStage::NotStartedYet | GameStage::NoMoreQuestions | GameStage::ScoredModeFinished => {}
        }
    }

    pub fn show_tolerance_marker(&self) -> bool {
        true
    }

    pub fn show_circle_marker(&self) -> bool {
        self.data.is_bayer || self.data.is_starname
    }

    pub fn get_question_distance_tolerance(&self) -> Deg<f32> {
        self.data.small_settings.correctness_threshold
    }

    pub fn allow_multiple_player_markers(&self) -> bool {
        false
    }

    pub fn add_marker_on_click(&self) -> bool {
        true
    }

    pub fn should_display_input(&self) -> bool {
        false
    }

    pub fn start_question(&mut self, _theme: &Theme, actions: &mut Vec<Action>) {
        self.state = Default::default();
        actions.push(Action::RemoveGameMarkers);
        actions.push(Action::DisableSingleRenderer(self.data.object_id));
    }

    fn render_display_question(&self, ui: &mut egui::Ui) {
        ui.heading("Find the object that is missing in the sky");
    }
}

pub fn generate_questions(objects: &[&crate::game::QuestionObject], small_settings: SmallSettings) -> Vec<questions::Question> {
    let mut questions: Vec<questions::Question> = Vec::with_capacity(objects.len());
    for object in objects {
        let mut possible_names = Vec::new();
        if let Some(designation) = &object.bayer_designation_raw {
            let names = crate::rendering::caspr::generate_name_combinations(designation, crate::rendering::caspr::SpecificName::None);
            possible_names.extend(names);
        }
        if let Some(designation) = object.caldwell_number {
            possible_names.push(format!("C{designation}"));
        }
        if let Some(designation) = &object.flamsteed_designation_raw {
            let names = crate::rendering::caspr::generate_name_combinations(designation, crate::rendering::caspr::SpecificName::None);
            possible_names.extend(names);
        }
        if let Some(designation) = &object.hd_number {
            possible_names.push(format!("HD{designation}"));
        }
        if let Some(designation) = &object.hipparcos_number {
            possible_names.push(format!("HIP{designation}"));
        }
        if let Some(designation) = &object.ic_number {
            possible_names.push(format!("IC{designation}"));
        }
        if let Some(designation) = &object.messier_number {
            possible_names.push(format!("M{designation}"));
        }
        if let Some(designation) = &object.ngc_number {
            possible_names.push(format!("NGC{designation}"));
        }
        for name in &object.proper_names_raw {
            let names = crate::rendering::caspr::generate_name_combinations(name, crate::rendering::caspr::SpecificName::None);
            possible_names.extend(names);
        }
        let question = Question {
            small_settings,
            ra: object.ra,
            dec: object.dec,
            possible_names,
            is_messier: object.messier_number.is_some(),
            is_caldwell: object.caldwell_number.is_some(),
            is_ngc: object.ngc_number.is_some(),
            is_ic: object.ic_number.is_some(),
            is_bayer: object.bayer_designation_full.is_some(),
            is_starname: matches!(object.object_type, crate::game::ObjectType::Star(_)),
            magnitude: object.mag,
            object_type: match &object.object_type {
                crate::game::ObjectType::Star(star_type) => star_type.display_name(),
                crate::game::ObjectType::Deepsky(deepsky_type) => deepsky_type.display_name(),
            },
            constellation_abbreviation: object.constellations_abbreviations.first().cloned().unwrap_or(String::from("Unknown")),
            images: object.images.clone(),
            object_id: object.object_id,
        };
        questions.push(questions::Question::MarkMissingObject(question));
    }
    questions
}
