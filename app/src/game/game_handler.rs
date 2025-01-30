use super::{game_settings, questions};
use crate::{
    enums::{self, GameStage, RendererCategory, StorageKeys},
    renderer::CellestialSphere,
    rendering::{
        caspr::markers::game_markers::{GameMarker, GameMarkerType},
        themes::Theme,
    },
};
use angle::Angle;
use eframe::egui;
use rand::Rng;
use std::collections::HashMap;

pub struct QuestionWindowData<'a> {
    pub cellestial_sphere: &'a mut CellestialSphere,
    pub theme: &'a Theme,
    pub game_question_opened: &'a mut bool,
    pub request_input_focus: &'a mut bool,
    pub add_marker_on_click: &'a mut bool,
    pub question_number_text: &'a String,
    pub game_stage: &'a mut GameStage,
    pub ctx: &'a eframe::egui::Context,
    pub start_next_question: &'a mut bool,
    pub score: &'a mut u32,
    pub possible_score: &'a mut u32,
    pub is_scored_mode: bool,
    pub current_question: usize,
    pub used_questions: &'a mut Vec<usize>,
    pub questions_settings: &'a questions::Settings,
    pub question_number: &'a mut usize,
}

pub struct QuestionCheckingData<'a> {
    pub cellestial_sphere: &'a mut CellestialSphere,
    pub theme: &'a Theme,
    pub game_stage: &'a mut GameStage,
    pub score: &'a mut u32,
    pub possible_score: &'a mut u32,
    pub is_scored_mode: bool,
    pub current_question: usize,
    pub used_questions: &'a mut Vec<usize>,
    pub add_marker_on_click: &'a mut bool,
    pub questions_settings: &'a questions::Settings,
    pub question_number: &'a mut usize,
    pub start_next_question: &'a mut bool,
}

pub trait QuestionTrait {
    fn render_window(&mut self, data: QuestionWindowData) -> Option<egui::InnerResponse<Option<()>>>;

    /// This function should handle cases where a generic action switches the question to the next part
    fn generic_to_next_part(&mut self, data: QuestionCheckingData);

    // fn check_answer(&self, game_handler: &mut GameHandler, cellestial_sphere: &mut crate::renderer::CellestialSphere, theme: &Theme);

    fn can_choose_as_next(&self, questions_settings: &questions::Settings, active_constellations: &mut HashMap<String, bool>) -> bool;

    fn reset(self: Box<Self>) -> Box<dyn QuestionTrait>;

    fn show_tolerance_marker(&self) -> bool;

    fn show_circle_marker(&self) -> bool;

    fn get_question_distance_tolerance(&self, game_handler: &GameHandler) -> angle::Deg<f32>;

    fn allow_multiple_player_markers(&self) -> bool;

    fn add_marker_on_click(&self) -> bool;

    fn should_display_input(&self) -> bool;

    fn start_question(&mut self, questions_settings: &questions::Settings, cellestial_sphere: &mut crate::renderer::CellestialSphere, theme: &Theme);

    fn get_display_question(&self) -> String;

    fn clone_box(&self) -> Box<dyn QuestionTrait>;
}

impl Clone for Box<dyn QuestionTrait> {
    fn clone(&self) -> Box<dyn QuestionTrait> {
        self.clone_box()
    }
}

#[derive(Clone)]
pub enum QuestionEnum {
    ObjectQuestion {
        name: String,
        ra: angle::Deg<f32>,
        dec: angle::Deg<f32>,
        is_messier: bool,
        is_caldwell: bool,
        is_ngc: bool,
        is_ic: bool,
        is_bayer: bool,
        is_starname: bool,
        magnitude: Option<f32>,
        object_type: String,
        constellation_abbreviation: String,
        images: Vec<crate::structs::image_info::ImageInfo>,
    },
    PositionQuestion {
        ra: angle::Deg<f32>,
        dec: angle::Deg<f32>,
    },
    ThisPointObject {
        possible_names: Vec<String>,
        ra: angle::Deg<f32>,
        dec: angle::Deg<f32>,
        is_messier: bool,
        is_caldwell: bool,
        is_ngc: bool,
        is_ic: bool,
        is_bayer: bool,
        is_starname: bool,
        magnitude: Option<f32>,
        object_type: String,
        constellation_abbreviation: String,
        images: Vec<crate::structs::image_info::ImageInfo>,
    },
    DistanceBetweenQuestion {
        /// (ra, dec)
        point1: (angle::Deg<f32>, angle::Deg<f32>),
        /// (ra, dec)
        point2: (angle::Deg<f32>, angle::Deg<f32>),
    },
    RAQuestion {
        ra: angle::Deg<f32>,
        dec: angle::Deg<f32>,
    },
    DECQuestion {
        ra: angle::Deg<f32>,
        dec: angle::Deg<f32>,
    },
    MagQuestion {
        ra: angle::Deg<f32>,
        dec: angle::Deg<f32>,
        mag: f32,
    },
    NoMoreQuestions,
}

pub struct GameHandler {
    pub current_question: usize,
    pub question_catalog: Vec<Box<dyn QuestionTrait>>,
    pub used_questions: Vec<usize>,

    pub add_marker_on_click: bool,
    pub stage: enums::GameStage,
    pub answer_image: Option<crate::structs::image_info::ImageInfo>,

    pub question_number: usize,
    pub question_number_text: String,

    pub answer_review_text_heading: String,
    pub answer_review_text: String,
    pub answer: String,

    pub guess_marker_positions: Vec<[angle::Rad<f32>; 2]>,

    pub game_settings: game_settings::GameSettings,
    pub questions_settings: questions::Settings,

    pub possible_no_of_questions: u32,
    pub score: u32,
    pub possible_score: u32,
    pub constellation_groups_settings: sg_game_constellations::GameConstellations,

    pub request_input_focus: bool,
    pub switch_to_next_question: bool,
}

impl GameHandler {
    pub fn increment_possible_score(&mut self, inc: u32) {
        self.possible_score += inc;
    }

    pub fn use_up_current_question(&mut self) {
        self.used_questions.push(self.current_question);
    }

    pub fn generic_to_next_part(&mut self, data: QuestionCheckingData) {
        self.question_catalog[self.current_question].generic_to_next_part(data)
    }

    pub fn render_question_window(&mut self, data: QuestionWindowData) -> Option<egui::InnerResponse<Option<()>>> {
        self.question_catalog[self.question_number].render_window(data)
    }
    pub fn init(cellestial_sphere: &mut CellestialSphere, storage: Option<&dyn eframe::Storage>) -> Self {
        let mut active_constellations = HashMap::new();
        for constellation_abbreviation in cellestial_sphere.constellations.keys() {
            active_constellations.insert(constellation_abbreviation.to_owned(), true);
        }
        if let Some(storage) = storage {
            if let Some(inactive_constellations) = storage.get_string(StorageKeys::GameInactiveConstellations.as_ref()) {
                let inactive_constellations = inactive_constellations.split('|');
                for inactive_constellation in inactive_constellations {
                    active_constellations.insert(inactive_constellation.to_string(), false);
                }
            }
        }
        let mut catalog: Vec<Box<dyn QuestionTrait>> = Vec::new();
        // catalog.push(QuestionEnum::NoMoreQuestions);
        for deepskies_group in cellestial_sphere.deepskies.values() {
            for deepsky in &deepskies_group.deepskies {
                let mut possible_names = Vec::new();
                let is_messier = deepsky.messier.is_some();
                let is_caldwell = deepsky.caldwell.is_some();
                let is_ngc = deepsky.ngc.is_some();
                let is_ic = deepsky.ic.is_some();
                let object_type = deepsky.object_type.clone().unwrap_or("Unknown".to_string());
                if let Some(messier_number) = &deepsky.messier {
                    let messier_name: String = format!("M{}", messier_number);
                    catalog.push(Box::new(questions::find_this_object::Question {
                        name: messier_name.to_owned(),
                        ra: deepsky.ra,
                        dec: deepsky.dec,
                        is_messier: true,
                        is_caldwell: false,
                        is_ngc: false,
                        is_ic: false,
                        is_bayer: false,
                        is_starname: false,
                        magnitude: None,
                        object_type: object_type.clone(),
                        constellation_abbreviation: deepsky.constellation.to_owned(),
                        images: deepsky.images.clone(),

                        state: Default::default(),
                    }));
                    possible_names.push(messier_name);
                }
                if let Some(caldwell_number) = &deepsky.caldwell {
                    let caldwell_name: String = format!("C {}", caldwell_number);
                    catalog.push(Box::new(questions::find_this_object::Question {
                        name: caldwell_name.to_owned(),
                        ra: deepsky.ra,
                        dec: deepsky.dec,
                        is_messier: false,
                        is_caldwell: true,
                        is_ngc: false,
                        is_ic: false,
                        is_bayer: false,
                        is_starname: false,
                        magnitude: None,
                        object_type: object_type.clone(),
                        constellation_abbreviation: deepsky.constellation.to_owned(),
                        images: deepsky.images.clone(),

                        state: Default::default(),
                    }));
                    possible_names.push(caldwell_name.to_owned());
                }
                if let Some(ngc_number) = &deepsky.ngc {
                    let ngc_name = format!("NGC {}", ngc_number);
                    catalog.push(Box::new(questions::find_this_object::Question {
                        name: ngc_name.to_owned(),
                        ra: deepsky.ra,
                        dec: deepsky.dec,
                        is_messier: false,
                        is_caldwell: false,
                        is_ngc: true,
                        is_ic: false,
                        is_bayer: false,
                        is_starname: false,
                        magnitude: None,
                        object_type: object_type.clone(),
                        constellation_abbreviation: deepsky.constellation.to_owned(),
                        images: deepsky.images.clone(),

                        state: Default::default(),
                    }));
                    possible_names.push(ngc_name.to_owned());
                }
                if let Some(ic_number) = &deepsky.ic {
                    let ic_name = format!("IC {}", ic_number);
                    catalog.push(Box::new(questions::find_this_object::Question {
                        name: ic_name.to_owned(),
                        ra: deepsky.ra,
                        dec: deepsky.dec,
                        is_messier: false,
                        is_caldwell: false,
                        is_ngc: false,
                        is_ic: true,
                        is_bayer: false,
                        is_starname: false,
                        magnitude: None,
                        object_type: object_type.clone(),
                        constellation_abbreviation: deepsky.constellation.to_owned(),
                        images: deepsky.images.clone(),

                        state: Default::default(),
                    }));
                    possible_names.push(ic_name.to_owned());
                }
                if !possible_names.is_empty() {
                    catalog.push(Box::new(questions::which_object_is_here::Question {
                        possible_names,
                        ra: deepsky.ra,
                        dec: deepsky.dec,
                        is_messier,
                        is_caldwell,
                        is_ngc,
                        is_ic,
                        is_bayer: false,
                        is_starname: false,
                        magnitude: None,
                        object_type: object_type.clone(),
                        constellation_abbreviation: deepsky.constellation.to_owned(),
                        images: deepsky.images.clone(),

                        state: Default::default(),
                    }));
                }
            }
        }
        for file in cellestial_sphere.star_names.values() {
            for starname in file {
                let mut possible_names: Vec<String> = vec![starname.name.to_owned()];
                catalog.push(Box::new(questions::find_this_object::Question {
                    ra: starname.ra,
                    dec: starname.dec,
                    is_messier: false,
                    is_caldwell: false,
                    is_ngc: false,
                    is_ic: false,
                    is_bayer: false,
                    is_starname: true,
                    magnitude: Some(starname.mag),
                    name: starname.name.to_owned(),
                    object_type: String::from("Star"),
                    constellation_abbreviation: starname.con.to_owned(),
                    images: Vec::new(),

                    state: Default::default(),
                }));
                let is_bayer: bool = match &starname.id_greek {
                    Some(id) => {
                        let name = format!("{} {}", id, starname.con);
                        possible_names.push(name.to_owned());
                        if let Some(id) = &starname.id {
                            possible_names.push(format!("{} {}", id, starname.con));
                        }
                        catalog.push(Box::new(questions::find_this_object::Question {
                            name,
                            ra: starname.ra,
                            dec: starname.dec,
                            is_messier: false,
                            is_caldwell: false,
                            is_ngc: false,
                            is_ic: false,
                            is_bayer: true,
                            is_starname: false,
                            magnitude: Some(starname.mag),
                            object_type: String::from("Star"),
                            constellation_abbreviation: starname.con.to_owned(),
                            images: Vec::new(),

                            state: Default::default(),
                        }));
                        catalog.push(Box::new(questions::guess_the_magnitude::Question {
                            ra: starname.ra,
                            dec: starname.dec,
                            mag: starname.mag,

                            state: Default::default(),
                        }));
                        true
                    }
                    None => false,
                };
                catalog.push(Box::new(questions::which_object_is_here::Question {
                    possible_names,
                    ra: starname.ra,
                    dec: starname.dec,
                    is_messier: false,
                    is_caldwell: false,
                    is_ngc: false,
                    is_ic: false,
                    is_bayer,
                    is_starname: true,
                    magnitude: Some(starname.mag),
                    object_type: String::from("Star"),
                    constellation_abbreviation: starname.con.to_owned(),
                    images: Vec::new(),

                    state: Default::default(),
                }));
            }
        }

        let mut rand = rand::thread_rng();
        for i in 1..catalog.len() {
            catalog.push(Box::new(questions::angular_separation::Question {
                point1: sg_geometry::generate_random_point(&mut rand),
                point2: sg_geometry::generate_random_point(&mut rand),

                state: Default::default(),
            }));
            let (ra, dec) = sg_geometry::generate_random_point(&mut rand);
            catalog.push(Box::new(questions::which_constellation_is_point_in::Question { ra, dec, state: Default::default() }));

            let (ra, dec) = sg_geometry::generate_random_point(&mut rand);
            if i % 2 == 0 {
                catalog.push(Box::new(questions::guess_ra_dec::DecQuestion { ra, dec, state: Default::default() }));
            } else {
                catalog.push(Box::new(questions::guess_ra_dec::RaQuestion { ra, dec, state: Default::default() }));
            }
        }

        // let entry = cellestial_sphere.markers.entry("game".to_string()).or_default();
        // *entry = Vec::new();
        // cellestial_sphere.init_single_renderer("markers", "game");

        let mut questions_settings = questions::Settings::default();
        if let Some(storage) = storage {
            if let Some(question_settings_str) = storage.get_string(StorageKeys::GameQuestionSettings.as_ref()) {
                match serde_json::from_str(&question_settings_str) {
                    Ok(data) => questions_settings = data,
                    Err(err) => log::error!("Failed to deserialize question game settings: {:?}", err),
                }
            }
        }

        let mut game_settings = game_settings::GameSettings::default();
        if let Some(storage) = storage {
            if let Some(game_settings_str) = storage.get_string(StorageKeys::GameSettings.as_ref()) {
                match serde_json::from_str(&game_settings_str) {
                    Ok(data) => game_settings = data,
                    Err(err) => log::error!("Failed to deserialize game settings: {:?}", err),
                }
            }
        }
        let constellation_groups_settings =
            sg_game_constellations::GameConstellations::load_from_storage(storage, &cellestial_sphere.constellations.values().map(|con| con.abbreviation.clone()).collect::<Vec<String>>());

        Self {
            current_question: 0,
            possible_no_of_questions: catalog.len() as u32,
            question_catalog: catalog,
            used_questions: Vec::new(),
            add_marker_on_click: false,
            stage: GameStage::NotStartedYet,
            answer_image: None,
            question_number: 0,
            question_number_text: String::new(),
            answer_review_text_heading: String::new(),
            answer_review_text: String::new(),
            answer: String::new(),
            guess_marker_positions: Vec::new(),
            questions_settings,
            game_settings,
            score: 0,
            possible_score: 0,
            constellation_groups_settings,
            request_input_focus: false,
            switch_to_next_question: false,
        }
    }
    pub fn evaluate_score(distance: angle::Deg<f32>) -> u32 {
        if distance < angle::Deg(0.2) {
            3
        } else if distance < angle::Deg(0.5) {
            2
        } else if distance < angle::Deg(1.0) {
            1
        } else {
            0
        }
    }

    pub fn next_question(&mut self, cellestial_sphere: &mut crate::renderer::CellestialSphere, theme: &Theme) {
        self.answer = String::new();
        let mut possible_questions: Vec<usize> = Vec::new();
        for question in 0..self.question_catalog.len() {
            if !self.used_questions.contains(&question) && self.question_catalog[question].can_choose_as_next(&self.questions_settings, &mut self.constellation_groups_settings.active_constellations) {
                possible_questions.push(question);
            }
        }

        if possible_questions.is_empty() {
            self.stage = GameStage::NoMoreQuestions;
        } else if self.game_settings.is_scored_mode && self.used_questions.len() as u32 > self.game_settings.no_of_questions {
            self.stage = GameStage::ScoredModeFinished;
        } else {
            self.current_question = possible_questions[rand::thread_rng().gen_range(0..possible_questions.len())];
            self.question_number_text = format!(
                "Question {}/{}",
                self.used_questions.len() + self.question_number + 1,
                possible_questions.len() + self.used_questions.len() + self.question_number
            );

            self.add_marker_on_click = self.question_catalog[self.current_question].add_marker_on_click();
            self.question_catalog[self.current_question].start_question(&self.questions_settings, cellestial_sphere, theme);
            self.request_input_focus = true;
            cellestial_sphere.init_single_renderer(RendererCategory::Markers, "game");
            self.stage = GameStage::Guessing;
        }
    }
    pub fn get_display_question(&self) -> String {
        match self.stage {
            GameStage::NoMoreQuestions => String::from("There are no more questions to be chosen from. You can either add more question packs from the game settings and click 'Next question', or return to the questions you already went through by clicking 'Reset and next question'."),
            GameStage::ScoredModeFinished => {
                let percentage = (self.score as f32) / (self.possible_score as f32) * 100.0;
                format!(
                    "Game over! Your score was {}/{}, that is {:.1}% of the maximum. Click Reset if you want to play a new game!",
                    self.score, self.possible_score, percentage
                )
            },
            _ => self.question_catalog[self.current_question].get_display_question()
        }
    }

    pub fn should_display_input(&self) -> bool {
        self.question_catalog[self.current_question].should_display_input()
    }

    pub fn no_more_questions(&self) -> bool {
        matches!(self.stage, GameStage::NoMoreQuestions | GameStage::ScoredModeFinished)
    }

    pub fn reset_used_questions(&mut self, _cellestial_sphere: &mut CellestialSphere) {
        self.used_questions = Vec::new();
        self.score = 0;
        self.possible_score = 0;
        self.question_number = 0;
        self.question_catalog = self
            .question_catalog
            .drain(..)
            .map(|question: Box<dyn QuestionTrait>| question.reset())
            .collect::<Vec<Box<dyn QuestionTrait>>>();
    }
    pub fn show_circle_marker(&self) -> bool {
        self.question_catalog[self.current_question].show_circle_marker()
    }

    pub fn show_tolerance_marker(&self) -> bool {
        self.question_catalog[self.current_question].show_tolerance_marker()
    }

    fn get_question_distance_tolerance(&self) -> angle::Deg<f32> {
        self.question_catalog[self.current_question].get_question_distance_tolerance(self)
    }

    pub fn allow_multiple_player_marker(&self) -> bool {
        self.question_catalog[self.current_question].allow_multiple_player_markers()
    }

    pub fn generate_player_markers(&self, marker_positions: &Vec<[angle::Rad<f32>; 2]>, theme: &Theme) -> Vec<GameMarker> {
        let mut markers = Vec::new();
        for &[dec, ra] in marker_positions {
            markers.push(GameMarker::new(
                GameMarkerType::Exact,
                ra.to_deg(),
                dec.to_deg(),
                2.0,
                5.0,
                self.show_circle_marker(),
                false,
                &theme.game_visuals.game_markers_colours,
            ));
            if self.show_tolerance_marker() {
                markers.push(GameMarker::new(
                    GameMarkerType::Tolerance,
                    ra.to_deg(),
                    dec.to_deg(),
                    2.0,
                    self.get_question_distance_tolerance().value(),
                    true,
                    true,
                    &theme.game_visuals.game_markers_colours,
                ));
            }
        }
        markers
    }

    pub fn get_possible_score(&self) -> u32 {
        self.possible_score
    }
}
