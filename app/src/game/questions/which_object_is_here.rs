use crate::enums::GameStage;
use crate::game::game_handler::{GameHandler, QuestionCheckingData, QuestionTrait, QuestionWindowData};
use crate::game::{game_handler, questions};
use crate::renderer::CellestialSphere;
use crate::rendering::caspr::markers::game_markers::{GameMarker, GameMarkerType};
use crate::rendering::themes::Theme;
use angle::Deg;
use eframe::egui;
use rand::Rng;
use std::collections::HashMap;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Settings {
    pub rotate_to_point: bool,
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
            if let Some(image) = &self.state.answer_image {
                ui.add(egui::Image::new(&image.path).max_width(600.0));
                if let Some(image_source) = &image.source {
                    ui.hyperlink_to("Image source", image_source);
                }
            }
            if ui.button("Next").clicked() {
                *data.start_next_question = true;
            }
            ui.label(data.question_number_text);
        })
    }

    fn check_answer(&mut self, data: QuestionCheckingData) {
        if !self.images.is_empty() {
            self.state.answer_image = Some(self.images[rand::thread_rng().gen_range(0..self.images.len())].clone());
        }
        let possible_names_edited = self.possible_names.iter().map(|name| name.replace(' ', "").to_lowercase()).collect::<Vec<String>>();
        let correct = possible_names_edited.contains(&self.state.answer.replace(' ', "").to_lowercase());
        self.state.answer_review_text_heading = format!(
            "{}orrect!",
            if correct {
                *data.score += 1;
                "C"
            } else {
                "Inc"
            }
        );
        self.state.answer_review_text = format!(
            "Your answer was: {}\nPossible answers: {}\nObject type: {}",
            self.state.answer,
            self.possible_names.join(", "),
            self.object_type
        );
        *data.possible_score += 1;
        if !data.questions_settings.what_is_this_object.replay_incorrect || correct {
            data.used_questions.push(data.current_question);
        } else {
            *data.question_number += 1;
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

    fn can_choose_as_next(&self, questions_settings: &super::Settings, active_constellations: &mut HashMap<String, bool>) -> bool {
        let mag = (self.magnitude).unwrap_or(-1.0);
        questions_settings.what_is_this_object.show
            && ((questions_settings.what_is_this_object.show_messiers && self.is_messier)
                || (questions_settings.what_is_this_object.show_caldwells && self.is_caldwell)
                || (questions_settings.what_is_this_object.show_ngcs && self.is_ngc)
                || (questions_settings.what_is_this_object.show_ics && self.is_ic)
                || (questions_settings.what_is_this_object.show_bayer && self.is_bayer)
                || (questions_settings.what_is_this_object.show_starnames && self.is_starname))
            && ((!self.is_bayer && !self.is_starname) || mag < questions_settings.what_is_this_object.magnitude_cutoff)
            && *active_constellations.entry(self.constellation_abbreviation.to_lowercase()).or_insert(true)
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

            state: State::default(),
        })
    }

    fn show_tolerance_marker(&self) -> bool {
        false
    }

    fn show_circle_marker(&self) -> bool {
        self.is_bayer || self.is_starname
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
        cellestial_sphere.game_markers.markers = if self.is_bayer || self.is_starname {
            vec![GameMarker::new(
                GameMarkerType::Task,
                self.ra,
                self.dec,
                2.0,
                5.0,
                true,
                false,
                &theme.game_visuals.game_markers_colours,
            )]
        } else {
            vec![GameMarker::new(
                GameMarkerType::Task,
                self.ra,
                self.dec,
                2.0,
                5.0,
                false,
                false,
                &theme.game_visuals.game_markers_colours,
            )]
        };
        if questions_settings.what_is_this_object.rotate_to_point {
            let final_vector = sg_geometry::get_point_vector(self.ra, self.dec, &nalgebra::Matrix3::<f32>::identity());
            cellestial_sphere.look_at_point(&final_vector);
            cellestial_sphere.init_renderers();
        }
    }

    fn get_display_question(&self) -> String {
        String::from("What is this object?")
    }

    fn clone_box(&self) -> Box<dyn game_handler::QuestionTrait> {
        Box::new(self.clone())
    }
}
