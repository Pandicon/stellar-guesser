use crate::enums::{GameStage, RendererCategory};
use crate::game::game_handler;
use crate::game::game_handler::{GameHandler, QuestionCheckingData, QuestionTrait, QuestionWindowData};
use crate::renderer::CellestialSphere;
use crate::rendering::caspr::markers::game_markers::{GameMarker, GameMarkerType};
use crate::rendering::themes::Theme;
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

    pub state: State,
}

impl Question {
    fn render_question_window(&mut self, data: QuestionWindowData) -> Option<egui::InnerResponse<Option<()>>> {
        egui::Window::new("Question").open(data.game_question_opened).show(data.ctx, |ui| {
            self.render_display_question(ui);
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
                    switch_to_next_part: data.switch_to_next_part,
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
            if let Some(image) = &self.state.answer_image {
                ui.add(egui::Image::new(&image.path).max_width(600.0));
                if let Some(image_source) = &image.source {
                    ui.hyperlink_to("Image source", image_source);
                }
            }
            if ui.button("Next").clicked() {
                *data.switch_to_next_part = true;
            }
            ui.label(data.question_number_text);
        })
    }

    fn check_answer(&mut self, data: QuestionCheckingData) {
        *data.add_marker_on_click = false;
        let markers = &mut data.cellestial_sphere.game_markers.markers;
        let mut correct = false;
        if !self.images.is_empty() {
            self.state.answer_image = Some(self.images[rand::thread_rng().gen_range(0..self.images.len())].clone());
        }
        let (answer_dec_text, answer_ra_text, distance, answer_review_text_heading) = if !markers.is_empty() {
            let answer_dec = markers[0].dec;
            let answer_ra = markers[0].ra;
            let distance = sg_geometry::angular_distance((self.ra.to_rad(), self.dec.to_rad()), (answer_ra.to_rad(), answer_dec.to_rad())).to_deg();
            if data.is_scored_mode {
                *data.score += GameHandler::evaluate_score(distance);
            }
            (
                answer_dec.value().to_string(),
                answer_ra.value().to_string(),
                distance.value().to_string(),
                if distance < self.small_settings.correctness_threshold {
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
            self.possible_names.join(", "),
            answer_dec_text,
            answer_ra_text,
            self.dec.value(),
            self.ra.value(),
            distance,
            if self.is_bayer || self.is_starname { "circle" } else { "cross" },
            self.object_type
        );
        markers.push(GameMarker::new(
            GameMarkerType::CorrectAnswer,
            self.ra,
            self.dec,
            2.0,
            5.0,
            self.is_bayer || self.is_starname,
            false,
            &data.theme.game_visuals.game_markers_colours,
        ));
        if !data.questions_settings.find_this_object.replay_incorrect || correct {
            data.used_questions.push(data.current_question);
        } else {
            *data.question_number += 1;
        }
        if self.small_settings.rotate_to_answer {
            let final_vector = sg_geometry::get_point_vector(self.ra, self.dec, &nalgebra::Matrix3::<f32>::identity());
            data.cellestial_sphere.look_at_point(&final_vector);
            data.cellestial_sphere.init_renderers();
        } else {
            data.cellestial_sphere.init_single_renderer_group(RendererCategory::Markers, "game");
        }
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
                self.check_answer(data);
            }
            GameStage::Checked => {
                *data.start_next_question = true;
                data.cellestial_sphere.enable_single_renderer(self.object_id);
            }
            GameStage::NotStartedYet | GameStage::NoMoreQuestions | GameStage::ScoredModeFinished => {}
        }
    }

    fn reset(self: Box<Self>) -> Box<dyn game_handler::QuestionTrait> {
        Box::new(Self {
            possible_names: self.possible_names,
            ra: self.ra,
            dec: self.dec,
            is_messier: self.is_messier,
            is_caldwell: self.is_caldwell,
            is_ngc: self.is_ngc,
            is_ic: self.is_ic,
            is_bayer: self.is_bayer,
            is_starname: self.is_starname,
            magnitude: self.magnitude,
            object_type: self.object_type,
            constellation_abbreviation: self.constellation_abbreviation,
            images: self.images,

            state: Default::default(),
            small_settings: self.small_settings,
            object_id: self.object_id,
        })
    }

    fn show_tolerance_marker(&self) -> bool {
        true
    }

    fn show_circle_marker(&self) -> bool {
        self.is_bayer || self.is_starname
    }

    fn get_question_distance_tolerance(&self) -> Deg<f32> {
        self.small_settings.correctness_threshold
    }

    fn allow_multiple_player_markers(&self) -> bool {
        false
    }

    fn add_marker_on_click(&self) -> bool {
        true
    }

    fn should_display_input(&self) -> bool {
        false
    }

    fn start_question(&mut self, cellestial_sphere: &mut CellestialSphere, _theme: &Theme) {
        self.state = Default::default();
        cellestial_sphere.game_markers.markers = Vec::new();
        cellestial_sphere.disable_single_renderer(self.object_id);
    }

    fn render_display_question(&self, ui: &mut egui::Ui) {
        ui.heading("Find the object that is missing in the sky");
    }

    fn clone_box(&self) -> Box<dyn game_handler::QuestionTrait> {
        Box::new(self.clone())
    }
}
