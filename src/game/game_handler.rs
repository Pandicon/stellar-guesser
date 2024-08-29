use std::{collections::HashMap, f32::consts::PI};

use crate::{
    enums::{self, GameStage, RendererCategory, StorageKeys},
    game::questions_settings,
    renderer::CellestialSphere,
    rendering::{
        caspr::markers::game_markers::{GameMarker, GameMarkerType},
        themes::Theme,
    },
};
use rand::Rng;

use crate::geometry;

use super::game_settings;

#[derive(Clone)]
pub enum Question {
    ObjectQuestion {
        name: String,
        ra: f32,
        dec: f32,
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
        ra: f32,
        dec: f32,
        possible_constellation_names: Vec<String>,
    },
    ThisPointObject {
        possible_names: Vec<String>,
        ra: f32,
        dec: f32,
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
        point1: (f32, f32),
        point2: (f32, f32),
    },
    RAQuestion {
        ra: f32,
        dec: f32,
    },
    DECQuestion {
        ra: f32,
        dec: f32,
    },
    MagQuestion {
        ra: f32,
        dec: f32,
        mag: f32,
    },
    NoMoreQuestions,
}

pub struct GameHandler {
    current_question: usize,
    question_catalog: Vec<Question>,
    used_questions: Vec<usize>,

    pub add_marker_on_click: bool,
    pub stage: enums::GameStage,
    pub answer_image: Option<crate::structs::image_info::ImageInfo>,

    pub question_number: usize,
    pub question_number_text: String,

    pub answer_review_text_heading: String,
    pub answer_review_text: String,
    pub answer: String,

    pub guess_marker_positions: Vec<[f32; 2]>,

    pub game_settings: game_settings::GameSettings,
    pub questions_settings: questions_settings::QuestionsSettings,

    pub possible_no_of_questions: u32,
    pub score: u32,
    possible_score: u32,
    pub active_constellations: HashMap<String, bool>,
    pub groups_active_constellations: HashMap<enums::GameLearningStage, HashMap<String, bool>>,
    pub active_constellations_groups: HashMap<enums::GameLearningStage, bool>,
    pub toggle_all_constellations: bool,
}

impl GameHandler {
    pub fn init(cellestial_sphere: &mut CellestialSphere, storage: &mut Option<crate::storage::Storage>) -> Self {
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
        let mut active_constellations_groups = HashMap::new();
        for group in [
            enums::GameLearningStage::NotStarted,
            enums::GameLearningStage::Learning,
            enums::GameLearningStage::Reviewing,
            enums::GameLearningStage::Learned,
        ] {
            active_constellations_groups.insert(group, true);
        }
        if let Some(storage) = storage {
            if let Some(inactive_constellations_groups) = storage.get_string(StorageKeys::GameInactiveConstellationGroups.as_ref()) {
                let inactive_groups = inactive_constellations_groups.split('|');
                for inactive_group in inactive_groups {
                    active_constellations_groups.insert(enums::GameLearningStage::from_string(inactive_group), false);
                }
            }
        }
        let mut groups_active_constellations = HashMap::new();
        for group in [
            enums::GameLearningStage::NotStarted,
            enums::GameLearningStage::Learning,
            enums::GameLearningStage::Reviewing,
            enums::GameLearningStage::Learned,
        ] {
            let mut group_active_constellations = HashMap::new();
            for constellation_abbreviation in cellestial_sphere.constellations.keys() {
                group_active_constellations.insert(constellation_abbreviation.to_owned(), false);
            }
            if let Some(storage) = storage {
                if let Some(active_constellations) = storage.get_string(&format!("{}_{}", StorageKeys::GameGroupActiveConstellellations, group)) {
                    let active_constellations = active_constellations.split('|');
                    for active_constellation in active_constellations {
                        group_active_constellations.insert(active_constellation.to_string(), true);
                    }
                }
            }
            groups_active_constellations.insert(group, group_active_constellations);
        }
        let mut catalog: Vec<Question> = Vec::new();
        catalog.push(Question::NoMoreQuestions);
        for deepskies_group in cellestial_sphere.deepskies.values() {
            for deepsky in &deepskies_group.deepskies {
                let mut possible_names = Vec::new();
                let is_messier = deepsky.messier.is_some();
                let is_caldwell = deepsky.caldwell.is_some();
                let is_ngc = deepsky.ngc.is_some();
                let is_ic = deepsky.ic.is_some();
                let object_type = deepsky.object_type.clone().unwrap_or("Unknown".to_string());
                if let Some(messier_name) = &deepsky.messier {
                    catalog.push(Question::ObjectQuestion {
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
                    });
                    possible_names.push(messier_name.to_owned());
                }
                if let Some(caldwell_number) = &deepsky.caldwell {
                    let caldwell_name: String = format!("C {}", caldwell_number);
                    catalog.push(Question::ObjectQuestion {
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
                    });
                    possible_names.push(caldwell_name.to_owned());
                }
                if let Some(ngc_number) = &deepsky.ngc {
                    let ngc_name = format!("NGC {}", ngc_number);
                    catalog.push(Question::ObjectQuestion {
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
                    });
                    possible_names.push(ngc_name.to_owned());
                }
                if let Some(ic_number) = &deepsky.ic {
                    let ic_name = format!("IC {}", ic_number);
                    catalog.push(Question::ObjectQuestion {
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
                    });
                    possible_names.push(ic_name.to_owned());
                }
                if !possible_names.is_empty() {
                    catalog.push(Question::ThisPointObject {
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
                    });
                }
            }
        }
        for file in cellestial_sphere.star_names.values() {
            for starname in file {
                let mut possible_names: Vec<String> = vec![starname.name.to_owned()];
                catalog.push(Question::ObjectQuestion {
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
                });
                let is_bayer: bool = match &starname.id_greek {
                    Some(id) => {
                        let name = format!("{} {}", id, starname.con);
                        possible_names.push(name.to_owned());
                        match &starname.id {
                            Some(id) => possible_names.push(format!("{} {}", id, starname.con)),
                            None => (),
                        }
                        catalog.push(Question::ObjectQuestion {
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
                        });
                        catalog.push(Question::MagQuestion {
                            ra: starname.ra,
                            dec: starname.dec,
                            mag: starname.mag,
                        });
                        true
                    }
                    None => false,
                };
                catalog.push(Question::ThisPointObject {
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
                })
            }
        }

        let mut rand = rand::thread_rng();
        for i in 1..catalog.len() {
            catalog.push(Question::DistanceBetweenQuestion {
                point1: geometry::generate_random_point(&mut rand),
                point2: geometry::generate_random_point(&mut rand),
            });
            let (ra, dec) = geometry::generate_random_point(&mut rand);
            let abbrev = cellestial_sphere.determine_constellation((ra, dec));
            let possible_constellation_names: Vec<String> = match cellestial_sphere.constellations.get(&abbrev) {
                None => vec![String::from("Undefined")],
                Some(constellation) => constellation.possible_names.to_owned(),
            };
            catalog.push(Question::PositionQuestion {
                ra,
                dec,
                possible_constellation_names,
            });

            let (ra, dec) = geometry::generate_random_point(&mut rand);
            if i % 2 == 0 {
                catalog.push(Question::DECQuestion { ra, dec });
            } else {
                catalog.push(Question::RAQuestion { ra, dec });
            }
        }

        // let entry = cellestial_sphere.markers.entry("game".to_string()).or_default();
        // *entry = Vec::new();
        // cellestial_sphere.init_single_renderer("markers", "game");

        let mut questions_settings = questions_settings::QuestionsSettings::default();
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
            active_constellations,
            groups_active_constellations,
            active_constellations_groups,
            toggle_all_constellations: true,
        }
    }
    pub fn evaluate_score(distance: f32) -> u32 {
        if distance < 0.2 {
            3
        } else if distance < 0.5 {
            2
        } else if distance < 1.0 {
            1
        } else {
            0
        }
    }

    pub fn check_answer(&mut self, cellestial_sphere: &mut crate::renderer::CellestialSphere, theme: &Theme) {
        self.stage = GameStage::Checked;
        self.add_marker_on_click = false;
        self.answer_image = None;
        let markers = &mut cellestial_sphere.game_markers.markers;
        match &self.question_catalog[self.current_question] {
            Question::ObjectQuestion {
                name,
                ra,
                dec,
                is_bayer,
                is_starname,
                object_type,
                images,
                ..
            } => {
                self.possible_score += 3;
                let mut correct = false;
                if !images.is_empty() {
                    self.answer_image = Some(images[rand::thread_rng().gen_range(0..images.len())].clone());
                }
                let (answer_dec_text, answer_ra_text, distance, answer_review_text_heading) = if !markers.is_empty() {
                    let answer_dec = markers[0].dec;
                    let answer_ra = markers[0].ra;
                    let distance = geometry::angular_distance((ra * PI / 180.0, dec * PI / 180.0), (answer_ra * PI / 180.0, answer_dec * PI / 180.0)) * 180.0 / PI;
                    if self.game_settings.is_scored_mode {
                        self.score += GameHandler::evaluate_score(distance);
                    }
                    (
                        answer_dec.to_string(),
                        answer_ra.to_string(),
                        distance.to_string(),
                        if distance < self.questions_settings.find_this_object.correctness_threshold {
                            correct = true;
                            String::from("Correct!")
                        } else {
                            format!("You were {} degrees away from {} !", (distance * 100.0).round() / 100.0, name)
                        },
                    )
                } else {
                    (String::from("-"), String::from("-"), String::from("-"), format!("You didn't guess where {} is", name))
                };
                self.answer_review_text_heading = answer_review_text_heading;
                self.answer_review_text = format!(
					"Your coordinates: [dec = {}; ra = {}]\nCorrect coordinates: [dec = {}; ra = {}]\nFully precise distance: {} degrees\nYou can see the correct place marked with a new {}.\nObject type: {}",
					answer_dec_text,
					answer_ra_text,
					dec,
					ra,
					distance,
					if *is_bayer || *is_starname { "circle" } else { "cross" },
					object_type
				);
                markers.push(GameMarker::new(
                    GameMarkerType::CorrectAnswer,
                    *ra,
                    *dec,
                    2.0,
                    5.0,
                    *is_bayer || *is_starname,
                    false,
                    &theme.game_visuals.game_markers_colours,
                ));
                if !self.questions_settings.find_this_object.replay_incorrect || correct {
                    self.used_questions.push(self.current_question);
                } else {
                    self.question_number += 1;
                }
            }
            Question::PositionQuestion { possible_constellation_names, .. } => {
                let possible_names_edited = possible_constellation_names.iter().map(|name| name.replace(' ', "").to_lowercase()).collect::<Vec<String>>();
                let correct = possible_names_edited.contains(&self.answer.replace(' ', "").to_lowercase());
                self.answer_review_text_heading = format!(
                    "{}orrect!",
                    if correct {
                        self.score += 1;
                        "C"
                    } else {
                        "Inc"
                    }
                );
                self.possible_score += 1;
                self.answer_review_text = format!("Your answer was: {}\nThe right answers were: {}", self.answer, possible_constellation_names.join(", "));
                self.used_questions.push(self.current_question);
            }
            Question::ThisPointObject {
                possible_names, object_type, images, ..
            } => {
                if !images.is_empty() {
                    self.answer_image = Some(images[rand::thread_rng().gen_range(0..images.len())].clone());
                }
                let possible_names_edited = possible_names.iter().map(|name| name.replace(' ', "").to_lowercase()).collect::<Vec<String>>();
                let correct = possible_names_edited.contains(&self.answer.replace(' ', "").to_lowercase());
                self.answer_review_text_heading = format!(
                    "{}orrect!",
                    if correct {
                        self.score += 1;
                        "C"
                    } else {
                        "Inc"
                    }
                );
                self.answer_review_text = format!("Your answer was: {}\nPossible answers: {}\nObject type: {}", self.answer, possible_names.join(", "), object_type);
                self.possible_score += 1;
                if !self.questions_settings.what_is_this_object.replay_incorrect || correct {
                    self.used_questions.push(self.current_question);
                } else {
                    self.question_number += 1;
                }
            }
            Question::DistanceBetweenQuestion { point1, point2 } => {
                let (ra1, dec1) = point1;
                let (ra2, dec2) = point2;
                let distance = geometry::angular_distance((ra1 * PI / 180.0, dec1 * PI / 180.0), (ra2 * PI / 180.0, dec2 * PI / 180.0)) * 180.0 / PI;
                let answer_dist: f32 = match self.answer.parse() {
                    Ok(answer) => {
                        self.answer_review_text_heading = format!("You were {:.1} degrees away!", distance - answer);
                        let error_percent = 1.0 - answer / distance;
                        self.answer_review_text = format!("The real distance was {:.1}°. Your error is equal to {:.1}% of the distance.", distance, error_percent * 100.0);
                        answer
                    }
                    Err(_) => {
                        self.answer_review_text_heading = "You didn't guess".to_string();
                        self.answer_review_text = format!("The real distance was {:.1}°.", distance);

                        0.0
                    }
                };
                if self.game_settings.is_scored_mode {
                    let error = 1.0 - (answer_dist / distance).abs();
                    if error < 0.03 {
                        self.score += 3;
                    } else if error < 0.05 {
                        self.score += 2;
                    } else if error < 0.1 {
                        self.score += 1;
                    }
                    self.possible_score += 3;
                }
                self.used_questions.push(self.current_question);
            }
            Question::RAQuestion { ra, .. } => {
                let answer_dist: f32 = match self.answer.parse::<f32>() {
                    Ok(answer) => {
                        self.answer_review_text_heading = format!("You were {:.1}h away!", (answer - ra/ 360.0 * 24.0).abs() );

                        self.answer_review_text = format!("The real right ascension was {:.1}h", ra / 360.0 * 24.0);
                        answer
                    }
                    Err(_) => {
                        self.answer_review_text_heading = "You didn't guess".to_string();
                        self.answer_review_text = format!("The real right ascension was {:.1}h.", ra / 360.0 * 24.0);

                        0.0
                    }
                };
                let error = (ra - answer_dist / 24.0 * 360.0).abs();

                if self.game_settings.is_scored_mode {
                    if error < 0.1 {
                        self.score += 3;
                    } else if error < 0.3 {
                        self.score += 2;
                    } else if error < 0.7 {
                        self.score += 1;
                    }
                    self.possible_score += 3;
                }
                self.used_questions.push(self.current_question);
            }
            Question::DECQuestion { dec, .. } => {
                let answer_dist: f32 = match self.answer.parse::<f32>() {
                    Ok(answer) => {
                        self.answer_review_text_heading = format!("You were {:.1}° away!", (answer - dec).abs());

                        self.answer_review_text = format!("The declination  was {:.1}°", dec);
                        answer
                    }
                    Err(_) => {
                        self.answer_review_text_heading = "You didn't guess".to_string();
                        self.answer_review_text = format!("The declination was {:.1}°.", dec);

                        0.0
                    }
                };
                let error = (dec - answer_dist).abs();

                if self.game_settings.is_scored_mode {
                    if error < 3.0 {
                        self.score += 3;
                    } else if error < 5.0 {
                        self.score += 2;
                    } else if error < 10.0 {
                        self.score += 1;
                    }
                    self.possible_score += 3;
                }
            }
            Question::MagQuestion { mag, .. } => {
                let answer_mag: f32 = match self.answer.parse() {
                    Ok(answer) => {
                        self.answer_review_text_heading = format!("You were {:.1} mag away!", answer - mag);

                        self.answer_review_text = format!("The  magnitude  was {:.1}.", mag);
                        answer
                    }
                    Err(_) => {
                        self.answer_review_text_heading = "You didn't guess".to_string();
                        self.answer_review_text = format!("The  magnitude  was {:.1}.", mag);

                        0.0
                    }
                };
                let error = (mag - answer_mag).abs();

                if self.game_settings.is_scored_mode {
                    if error < 3.0 {
                        self.score += 3;
                    } else if error < 5.0 {
                        self.score += 2;
                    } else if error < 10.0 {
                        self.score += 1;
                    }
                    self.possible_score += 3;
                }
                self.used_questions.push(self.current_question);
            }
            Question::NoMoreQuestions => {}
        }
        cellestial_sphere.init_single_renderer(RendererCategory::Markers, "game");
    }

    pub fn next_question(&mut self, cellestial_sphere: &mut crate::renderer::CellestialSphere, theme: &Theme) {
        self.answer = String::new();
        let mut possible_questions: Vec<usize> = Vec::new();
        for question in 0..self.question_catalog.len() {
            if !self.used_questions.contains(&question) {
                match &self.question_catalog[question] {
                    Question::ObjectQuestion {
                        is_messier,
                        is_caldwell,
                        is_ngc,
                        is_ic,
                        is_bayer,
                        is_starname,
                        magnitude,
                        constellation_abbreviation,
                        ..
                    } => {
                        let mag = (*magnitude).unwrap_or(-1.0); // TODO: Shouldn't a default magnitude be something else?
                        if self.questions_settings.find_this_object.show
                            && ((self.questions_settings.find_this_object.show_messiers && *is_messier)
                                || (self.questions_settings.find_this_object.show_caldwells && *is_caldwell)
                                || (self.questions_settings.find_this_object.show_ngcs && *is_ngc)
                                || (self.questions_settings.find_this_object.show_ics && *is_ic)
                                || (self.questions_settings.find_this_object.show_bayer && *is_bayer)
                                || (self.questions_settings.find_this_object.show_starnames && *is_starname))
                            && ((!*is_bayer && !*is_starname) || mag < self.questions_settings.find_this_object.magnitude_cutoff)
                            && *self.active_constellations.entry(constellation_abbreviation.to_lowercase()).or_insert(true)
                        {
                            possible_questions.push(question);
                        }
                    }
                    Question::PositionQuestion { .. } => {
                        if self.questions_settings.what_constellation_is_this_point_in.show {
                            possible_questions.push(question);
                        }
                    }
                    Question::ThisPointObject {
                        is_messier,
                        is_caldwell,
                        is_ngc,
                        is_ic,
                        is_bayer,
                        is_starname,
                        magnitude,
                        constellation_abbreviation,
                        ..
                    } => {
                        let mag = (*magnitude).unwrap_or(-1.0);
                        if self.questions_settings.what_is_this_object.show
                            && ((self.questions_settings.what_is_this_object.show_messiers && *is_messier)
                                || (self.questions_settings.what_is_this_object.show_caldwells && *is_caldwell)
                                || (self.questions_settings.what_is_this_object.show_ngcs && *is_ngc)
                                || (self.questions_settings.what_is_this_object.show_ics && *is_ic)
                                || (self.questions_settings.what_is_this_object.show_bayer && *is_bayer)
                                || (self.questions_settings.what_is_this_object.show_starnames && *is_starname))
                            && ((!*is_bayer && !*is_starname) || mag < self.questions_settings.what_is_this_object.magnitude_cutoff)
                            && *self.active_constellations.entry(constellation_abbreviation.to_lowercase()).or_insert(true)
                        {
                            possible_questions.push(question);
                        }
                    }
                    Question::DistanceBetweenQuestion { point1: _point1, point2: _point2 } => {
                        if self.questions_settings.angular_separation.show {
                            possible_questions.push(question);
                        }
                    }
                    Question::DECQuestion { .. } => {
                        if self.questions_settings.guess_rad_dec.show {
                            possible_questions.push(question);
                        }
                    }
                    Question::RAQuestion { .. } => {
                        if self.questions_settings.guess_rad_dec.show {
                            possible_questions.push(question);
                        }
                    }
                    Question::MagQuestion { mag, .. } => {
                        if self.questions_settings.guess_the_magnitude.show && *mag < self.questions_settings.guess_the_magnitude.magnitude_cutoff {
                            possible_questions.push(question)
                        }
                    }
                    Question::NoMoreQuestions => {}
                }
            }
        }

        if possible_questions.is_empty() || (self.game_settings.is_scored_mode && self.used_questions.len() as u32 > self.game_settings.no_of_questions) {
            self.current_question = 0;
        } else {
            self.current_question = possible_questions[rand::thread_rng().gen_range(0..possible_questions.len())];
            self.question_number_text = format!(
                "Question {}/{}",
                self.used_questions.len() + self.question_number + 1,
                possible_questions.len() + self.used_questions.len() + self.question_number
            );

            let mut markers = Vec::new();
            self.add_marker_on_click = match self.question_catalog[self.current_question] {
                Question::ObjectQuestion { .. } => {
                    markers = Vec::new();
                    true
                }
                Question::PositionQuestion { ra, dec, .. } => {
                    markers = vec![GameMarker::new(GameMarkerType::Task, ra, dec, 2.0, 5.0, false, false, &theme.game_visuals.game_markers_colours)];
                    false
                }
                Question::ThisPointObject { ra, dec, is_bayer, is_starname, .. } => {
                    markers = if is_bayer || is_starname {
                        vec![GameMarker::new(GameMarkerType::Task, ra, dec, 2.0, 5.0, true, false, &theme.game_visuals.game_markers_colours)]
                    } else {
                        vec![GameMarker::new(GameMarkerType::Task, ra, dec, 2.0, 5.0, false, false, &theme.game_visuals.game_markers_colours)]
                    };
                    false
                }
                Question::DistanceBetweenQuestion { point1, point2 } => {
                    let (ra1, dec1) = point1;
                    let (ra2, dec2) = point2;
                    markers = vec![
                        GameMarker::new(GameMarkerType::Task, ra1, dec1, 2.0, 5.0, false, false, &theme.game_visuals.game_markers_colours),
                        GameMarker::new(GameMarkerType::Task, ra2, dec2, 2.0, 5.0, false, false, &theme.game_visuals.game_markers_colours),
                    ];
                    false
                }
                Question::DECQuestion { ra, dec } => {
                    markers = vec![GameMarker::new(GameMarkerType::Task, ra, dec, 2.0, 5.0, false, false, &theme.game_visuals.game_markers_colours)];
                    false
                }
                Question::RAQuestion { ra, dec } => {
                    markers = vec![GameMarker::new(GameMarkerType::Task, ra, dec, 2.0, 5.0, false, false, &theme.game_visuals.game_markers_colours)];
                    false
                }
                Question::MagQuestion { ra, dec, .. } => {
                    markers = vec![GameMarker::new(GameMarkerType::Task, ra, dec, 2.0, 5.0, true, false, &theme.game_visuals.game_markers_colours)];
                    false
                }
                Question::NoMoreQuestions => false,
            };
            cellestial_sphere.game_markers.markers = markers;
            cellestial_sphere.init_single_renderer(RendererCategory::Markers, "game");
        }
        self.stage = GameStage::Guessing;
    }
    pub fn get_display_question(&self) -> String {
        match &self.question_catalog[self.current_question] {
            Question::ObjectQuestion { name, .. } => format!("Find {}.", name),
            Question::PositionQuestion { .. } => String::from("What constellation does this point lie in?"),
            Question::ThisPointObject { .. } => String::from("What is this object?"),
            Question::DistanceBetweenQuestion { .. } => String::from("What is the angular distance between these markers? "),
            Question::NoMoreQuestions => {
                if self.game_settings.is_scored_mode {
                    let percentage = (self.score as f32) / (self.possible_score as f32) * 100.0;
                    format!(
                        "Game over! Your score was {}/{}, that is {:.1}% of the maximum. Click Reset if you want to play a new game!",
                        self.score, self.possible_score, percentage
                    )
                } else {
                    String::from("There are no more questions to be chosen from. You can either add more question packs from the game settings and click 'Next question', or return the questions you already went through by clicking 'Reset and next question'.")
                }
            }
            Question::DECQuestion { .. } => String::from("What is the declination of this point?"),
            Question::RAQuestion { .. } => String::from("What is the right ascension of this point?"),
            Question::MagQuestion { .. } => String::from("What is the magnitude of this star? "),
        }
    }

    pub fn should_display_input(&self) -> bool {
        match &self.question_catalog[self.current_question] {
            Question::ObjectQuestion { .. } | Question::NoMoreQuestions => false,
            Question::PositionQuestion { .. }
            | Question::ThisPointObject { .. }
            | Question::DistanceBetweenQuestion { .. }
            | Question::DECQuestion { .. }
            | Question::RAQuestion { .. }
            | Question::MagQuestion { .. } => true,
        }
    }

    pub fn no_more_questions(&self) -> bool {
        match &self.question_catalog[self.current_question] {
            Question::NoMoreQuestions => true,
            Question::ObjectQuestion { .. }
            | Question::PositionQuestion { .. }
            | Question::ThisPointObject { .. }
            | Question::DistanceBetweenQuestion { .. }
            | Question::DECQuestion { .. }
            | Question::RAQuestion { .. }
            | Question::MagQuestion { .. } => false,
        }
    }

    pub fn reset_used_questions(&mut self, cellestial_sphere: &mut CellestialSphere) {
        self.used_questions = Vec::new();
        self.score = 0;
        self.possible_score = 0;
        self.question_number = 0;
        let old_catalog = self.question_catalog.to_vec();
        self.question_catalog = old_catalog
            .into_iter()
            .map(|question| match question {
                Question::NoMoreQuestions | Question::ObjectQuestion { .. } | Question::ThisPointObject { .. } | Question::MagQuestion { .. } => question,
                Question::DECQuestion { .. } => {
                    let (ra, dec) = geometry::generate_random_point(&mut rand::thread_rng());
                    Question::DECQuestion { ra, dec }
                }
                Question::RAQuestion { .. } => {
                    let (ra, dec) = geometry::generate_random_point(&mut rand::thread_rng());
                    Question::RAQuestion { ra, dec }
                }
                Question::PositionQuestion { .. } => {
                    let (ra, dec) = geometry::generate_random_point(&mut rand::thread_rng());
                    let abbrev = cellestial_sphere.determine_constellation((ra, dec));
                    let possible_constellation_names: Vec<String> = match cellestial_sphere.constellations.get(&abbrev) {
                        None => vec![String::from("Undefined")],
                        Some(constellation) => constellation.possible_names.to_owned(),
                    };
                    Question::PositionQuestion {
                        ra,
                        dec,
                        possible_constellation_names,
                    }
                }
                Question::DistanceBetweenQuestion { .. } => Question::DistanceBetweenQuestion {
                    point1: geometry::generate_random_point(&mut rand::thread_rng()),
                    point2: geometry::generate_random_point(&mut rand::thread_rng()),
                },
            })
            .collect();
    }
    pub fn show_circle_marker(&self) -> bool {
        match &self.question_catalog[self.current_question] {
            Question::NoMoreQuestions | Question::PositionQuestion { .. } | Question::DistanceBetweenQuestion { .. } | Question::DECQuestion { .. } | Question::RAQuestion { .. } => false,
            Question::ObjectQuestion { is_bayer, is_starname, .. } | Question::ThisPointObject { is_bayer, is_starname, .. } => *is_bayer || *is_starname,
            Question::MagQuestion { .. } => true,
        }
    }

    pub fn show_tolerance_marker(&self) -> bool {
        match &self.question_catalog[self.current_question] {
            Question::NoMoreQuestions
            | Question::PositionQuestion { .. }
            | Question::DistanceBetweenQuestion { .. }
            | Question::DECQuestion { .. }
            | Question::RAQuestion { .. }
            | Question::MagQuestion { .. }
            | Question::ThisPointObject { .. } => false,
            Question::ObjectQuestion { .. } => true,
        }
    }

    fn get_question_distance_tolerance(&self) -> f32 {
        match &self.question_catalog[self.current_question] {
            Question::NoMoreQuestions
            | Question::PositionQuestion { .. }
            | Question::DistanceBetweenQuestion { .. }
            | Question::DECQuestion { .. }
            | Question::RAQuestion { .. }
            | Question::MagQuestion { .. }
            | Question::ThisPointObject { .. } => 0.0,
            Question::ObjectQuestion { .. } => self.questions_settings.find_this_object.correctness_threshold,
        }
    }

    pub fn allow_multiple_player_marker(&self) -> bool {
        match &self.question_catalog[self.current_question] {
            Question::NoMoreQuestions
            | Question::PositionQuestion { .. }
            | Question::DistanceBetweenQuestion { .. }
            | Question::DECQuestion { .. }
            | Question::RAQuestion { .. }
            | Question::MagQuestion { .. }
            | Question::ThisPointObject { .. }
            | Question::ObjectQuestion { .. } => false,
        }
    }

    pub fn generate_player_markers(&self, marker_positions: &Vec<[f32; 2]>, theme: &Theme) -> Vec<GameMarker> {
        let mut markers = Vec::new();
        for &[dec, ra] in marker_positions {
            markers.push(GameMarker::new(
                GameMarkerType::Exact,
                ra / PI * 180.0,
                dec / PI * 180.0,
                2.0,
                5.0,
                self.show_circle_marker(),
                false,
                &theme.game_visuals.game_markers_colours,
            ));
            if self.show_tolerance_marker() {
                markers.push(GameMarker::new(
                    GameMarkerType::Tolerance,
                    ra / PI * 180.0,
                    dec / PI * 180.0,
                    2.0,
                    self.get_question_distance_tolerance(),
                    true,
                    true,
                    &theme.game_visuals.game_markers_colours,
                ));
            }
        }
        markers
    }
}
