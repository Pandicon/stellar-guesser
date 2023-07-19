use std::{collections::HashMap, f32::consts::PI};

use crate::{caspr::CellestialSphere, enums, markers::Marker};
use eframe::epaint::Color32;
use rand::Rng;

#[path = "../geometry.rs"]
mod geometry;

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
		constellation_abbreviation: String,
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
		constellation_abbreviation: String,
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
	NoMoreQuestions,
}

pub struct QuestionSettings {
	pub show_messiers: bool,
	pub show_caldwells: bool,
	pub show_ngcs: bool,
	pub show_ics: bool,
	pub show_bayer: bool,
	pub show_starnames: bool,
	pub magnitude_cutoff: f32,
	pub replay_incorrect: bool,
}

impl Default for QuestionSettings {
	fn default() -> Self {
		Self {
			show_messiers: true,
			show_caldwells: true,
			show_ngcs: true,
			show_ics: true,
			show_bayer: true,
			show_starnames: true,
			magnitude_cutoff: 6.0,
			replay_incorrect: true,
		}
	}
}

pub struct GameHandler {
	current_question: usize,
	question_catalog: Vec<Question>,
	used_questions: Vec<usize>,

	pub add_marker_on_click: bool,
	/// 0 = guessing, 1 = checked, 2 = not started yet
	pub stage: usize,

	pub answer_review_text_heading: String,
	pub answer_review_text: String,
	pub answer: String,

	pub object_question_settings: QuestionSettings,
	pub this_point_object_question_settings: QuestionSettings,
	pub show_object_questions: bool,
	pub show_positions_questions: bool,
	pub show_this_point_object_questions: bool,
	pub show_distance_between_questions: bool,
	pub no_of_questions: u32,
	pub possible_no_of_questions: u32,
	pub is_scored_mode: bool,
	pub score: u32,
	possible_score: u32,
	pub show_radecquestions: bool,
	pub active_constellations: HashMap<String, bool>,
	pub groups_active_constellations: HashMap<enums::GameLearningStage, HashMap<String, bool>>,
	pub active_constellations_groups: HashMap<enums::GameLearningStage, bool>,
	pub toggle_all_constellations: bool,
}

impl GameHandler {
	pub fn init(cellestial_sphere: &mut CellestialSphere, storage: Option<&dyn eframe::Storage>) -> Self {
		let mut active_constellations = HashMap::new();
		for constellation_abbreviation in cellestial_sphere.constellations.keys() {
			active_constellations.insert(constellation_abbreviation.to_owned(), true);
		}
		if let Some(storage) = storage {
			if let Some(inactive_constellations) = storage.get_string("game_inactive_constellations") {
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
			if let Some(inactive_constellations_groups) = storage.get_string("inactive_constellations_groups") {
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
				if let Some(active_constellations) = storage.get_string(&format!("game_group_active_constellations_{}", group)) {
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
		for file in cellestial_sphere.deepskies.values() {
			for deepsky in file {
				let mut possible_names = Vec::new();
				let is_messier = deepsky.messier.is_some();
				let is_caldwell = deepsky.caldwell.is_some();
				let is_ngc = deepsky.ngc.is_some();
				let is_ic = deepsky.ic.is_some();
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
						constellation_abbreviation: deepsky.constellation.to_owned(),
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
						constellation_abbreviation: deepsky.constellation.to_owned(),
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
						constellation_abbreviation: deepsky.constellation.to_owned(),
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
						constellation_abbreviation: deepsky.constellation.to_owned(),
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
						constellation_abbreviation: deepsky.constellation.to_owned(),
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
					constellation_abbreviation: starname.con.to_owned(),
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
							constellation_abbreviation: starname.con.to_owned(),
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
					constellation_abbreviation: starname.con.to_owned(),
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

		Self {
			current_question: 0,
			possible_no_of_questions: catalog.len() as u32,
			question_catalog: catalog,
			used_questions: vec![0], // Don't allow it to draw the "No questions left question"
			add_marker_on_click: false,
			stage: 2,
			answer_review_text_heading: String::new(),
			answer_review_text: String::new(),
			answer: String::new(),
			object_question_settings: QuestionSettings::default(),
			this_point_object_question_settings: QuestionSettings::default(),
			show_object_questions: true,
			show_positions_questions: true,
			show_this_point_object_questions: true,
			no_of_questions: 15,
			is_scored_mode: false,
			score: 0,
			possible_score: 0,
			show_distance_between_questions: true,
			show_radecquestions: true,
			active_constellations,
			groups_active_constellations,
			active_constellations_groups,
			toggle_all_constellations: true,
		}
	}
	pub fn evaluate_score(distance: f32) -> u32 {
		if distance < 0.2 {
			return 3;
		} else if distance < 0.5 {
			return 2;
		} else if distance < 1.0 {
			return 1;
		} else {
			return 0;
		}
	}

	pub fn check_answer(&mut self, cellestial_sphere: &mut crate::caspr::CellestialSphere) {
		self.stage = 1;
		self.add_marker_on_click = false;
		let entry = cellestial_sphere.markers.entry("game".to_string()).or_default();
		match &self.question_catalog[self.current_question] {
			Question::ObjectQuestion {
				name, ra, dec, is_bayer, is_starname, ..
			} => {
				self.possible_score += 3;
				let mut correct = false;
				let (answer_dec_text, answer_ra_text, distance, answer_review_text_heading) = if !entry.is_empty() {
					let answer_dec = entry[0].dec;
					let answer_ra = entry[0].ra;
					let distance = geometry::angular_distance((ra * PI / 180.0, dec * PI / 180.0), (answer_ra * PI / 180.0, answer_dec * PI / 180.0)) * 180.0 / PI;
					if self.is_scored_mode {
						self.score += GameHandler::evaluate_score(distance);
					}
					(
						answer_dec.to_string(),
						answer_ra.to_string(),
						distance.to_string(),
						if distance < 0.2 {
							correct = true;
							String::from("Correct!")
						} else {
							format!("You were {} degrees away from {} !", (distance * 100.0).round() / 100.0, name)
						},
					)
				} else {
					(String::from("-"), String::from("-"), String::from("-"), String::from(format!("You didn't guess where {} is", name)))
				};
				self.answer_review_text_heading = answer_review_text_heading;
				self.answer_review_text = format!(
					"Your coordinates: [dec = {}; ra = {}]\nCorrect coordinates: [dec = {}; ra = {}]\nFully precise distance: {} degrees\nYou can see the correct place marked with a yellow {}.",
					answer_dec_text,
					answer_ra_text,
					dec,
					ra,
					distance,
					if *is_bayer || *is_starname { "circle" } else { "cross" }
				);
				entry.push(Marker::new(*ra, *dec, Color32::YELLOW, 2.0, 5.0, *is_bayer || *is_starname, false));
				if !self.object_question_settings.replay_incorrect || correct {
					self.used_questions.push(self.current_question);
				}
			}
			Question::PositionQuestion { possible_constellation_names, .. } => {
				let possible_names_edited = possible_constellation_names.iter().map(|name| name.replace(" ", "").to_lowercase()).collect::<Vec<String>>();
				let correct = possible_names_edited.contains(&self.answer.replace(" ", "").to_lowercase());
				self.answer_review_text_heading = format!(
					"{}orrect!",
					if correct {
						self.score += 1;
						"C"
					} else {
						"Inc"
					}
				);
				self.answer_review_text = format!("Your answer was: {}\nThe right answers were: {}", self.answer, possible_constellation_names.join(", "));
				self.used_questions.push(self.current_question);
			}
			Question::ThisPointObject { possible_names, .. } => {
				let possible_names_edited = possible_names.iter().map(|name| name.replace(" ", "").to_lowercase()).collect::<Vec<String>>();
				let correct = possible_names_edited.contains(&self.answer.replace(" ", "").to_lowercase());
				self.answer_review_text_heading = format!(
					"{}orrect!",
					if correct {
						self.score += 1;
						"C"
					} else {
						"Inc"
					}
				);
				self.answer_review_text = format!("Your answer was: {}\nPossible answers: {}", self.answer, possible_names.join(", "));
				self.possible_score += 1;
				if !self.this_point_object_question_settings.replay_incorrect || correct {
					self.used_questions.push(self.current_question);
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
						self.answer_review_text_heading = format!("You didn't guess");
						self.answer_review_text = format!("The real distance was {:.1}°.", distance);

						0.0
					}
				};
				if self.is_scored_mode {
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
				let answer_dist: f32 = match self.answer.parse() {
					Ok(answer) => {
						self.answer_review_text_heading = format!("You were {:.1}° away!", answer - ra);

						self.answer_review_text = format!("The real right ascension was {:.1}°", ra);
						answer
					}
					Err(_) => {
						self.answer_review_text_heading = format!("You didn't guess");
						self.answer_review_text = format!("The real right ascension was {:.1}°.", ra);

						0.0
					}
				};
				let error = (ra - answer_dist).abs();

				if self.is_scored_mode {
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
			Question::DECQuestion { dec, .. } => {
				let answer_dist: f32 = match self.answer.parse() {
					Ok(answer) => {
						self.answer_review_text_heading = format!("You were {:.1}° away!", answer - dec);

						self.answer_review_text = format!("The declination  was {:.1}°", dec);
						answer
					}
					Err(_) => {
						self.answer_review_text_heading = format!("You didn't guess");
						self.answer_review_text = format!("The declination was {:.1}°.", dec);

						0.0
					}
				};
				let error = (dec - answer_dist).abs();

				if self.is_scored_mode {
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
		cellestial_sphere.init_single_renderer("markers", "game");
	}

	pub fn next_question(&mut self, cellestial_sphere: &mut crate::caspr::CellestialSphere) {
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
						name,
						..
					} => {
						let mag = match *magnitude {
							Some(mag) => mag,
							None => -1.0,
						};
						if self.show_object_questions
							&& ((self.object_question_settings.show_messiers && *is_messier)
								|| (self.object_question_settings.show_caldwells && *is_caldwell)
								|| (self.object_question_settings.show_ngcs && *is_ngc)
								|| (self.object_question_settings.show_ics && *is_ic)
								|| (self.object_question_settings.show_bayer && *is_bayer)
								|| (self.object_question_settings.show_starnames && *is_starname))
							&& ((!*is_bayer && !*is_starname) || mag < self.object_question_settings.magnitude_cutoff)
							&& *self.active_constellations.entry(constellation_abbreviation.to_lowercase()).or_insert(true)
						{
							possible_questions.push(question);
						}
					}
					Question::PositionQuestion { .. } => {
						if self.show_positions_questions {
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
						let mag = match *magnitude {
							Some(mag) => mag,
							None => -1.0,
						};
						if self.show_this_point_object_questions
							&& ((self.this_point_object_question_settings.show_messiers && *is_messier)
								|| (self.this_point_object_question_settings.show_caldwells && *is_caldwell)
								|| (self.this_point_object_question_settings.show_ngcs && *is_ngc)
								|| (self.this_point_object_question_settings.show_ics && *is_ic)
								|| (self.this_point_object_question_settings.show_bayer && *is_bayer)
								|| (self.this_point_object_question_settings.show_starnames && *is_starname))
							&& ((!*is_bayer && !*is_starname) || mag < self.this_point_object_question_settings.magnitude_cutoff)
							&& *self.active_constellations.entry(constellation_abbreviation.to_lowercase()).or_insert(true)
						{
							possible_questions.push(question);
						}
					}
					Question::DistanceBetweenQuestion { point1: _point1, point2: _point2 } => {
						if self.show_distance_between_questions {
							possible_questions.push(question);
						}
					}
					Question::DECQuestion { .. } => {
						if self.show_radecquestions {
							possible_questions.push(question);
						}
					}
					Question::RAQuestion { .. } => {
						if self.show_radecquestions {
							possible_questions.push(question);
						}
					}
					Question::NoMoreQuestions => {}
				}
			}
		}

		if possible_questions.is_empty() || (self.is_scored_mode && self.used_questions.len() as u32 > self.no_of_questions) {
			self.current_question = 0;
		} else {
			self.current_question = possible_questions[rand::thread_rng().gen_range(0..possible_questions.len())];

			let entry = cellestial_sphere.markers.entry("game".to_string()).or_default();
			self.add_marker_on_click = match self.question_catalog[self.current_question] {
				Question::ObjectQuestion { .. } => {
					*entry = Vec::new();
					true
				}
				Question::PositionQuestion { ra, dec, .. } => {
					*entry = vec![Marker::new(ra, dec, Color32::YELLOW, 2.0, 5.0, false, false)];
					false
				}
				Question::ThisPointObject { ra, dec, is_bayer, is_starname, .. } => {
					*entry = if is_bayer || is_starname {
						vec![Marker::new(ra, dec, Color32::YELLOW, 2.0, 5.0, true, false)]
					} else {
						vec![Marker::new(ra, dec, Color32::YELLOW, 2.0, 5.0, false, false)]
					};
					false
				}
				Question::DistanceBetweenQuestion { point1, point2 } => {
					let (ra1, dec1) = point1;
					let (ra2, dec2) = point2;
					*entry = vec![
						Marker::new(ra1, dec1, Color32::GREEN, 2.0, 5.0, false, false),
						Marker::new(ra2, dec2, Color32::GREEN, 2.0, 5.0, false, false),
					];
					false
				}
				Question::DECQuestion { ra, dec } => {
					*entry = vec![Marker::new(ra, dec, Color32::GREEN, 2.0, 5.0, false, false)];
					false
				}
				Question::RAQuestion { ra, dec } => {
					*entry = vec![Marker::new(ra, dec, Color32::GREEN, 2.0, 5.0, false, false)];
					false
				}

				Question::NoMoreQuestions => false,
			};
			cellestial_sphere.init_single_renderer("markers", "game");
		}
		self.stage = 0;
	}
	pub fn get_display_question(&self) -> String {
		match &self.question_catalog[self.current_question] {
			Question::ObjectQuestion { name, .. } => {
				return String::from(format!("Find {}.", name));
			}
			Question::PositionQuestion { .. } => {
				return String::from("What constellation does this point lie in?");
			}
			Question::ThisPointObject { .. } => {
				return String::from("What is this object?");
			}
			Question::DistanceBetweenQuestion { .. } => {
				return String::from("What is the angular distance between these markers? ");
			}
			Question::NoMoreQuestions => {
				if self.is_scored_mode {
					let percentage = (self.score as f32) / (self.possible_score as f32) * 100.0;
					String::from(format!(
						"Game over! Your score was {}/{}, that is {:.1}% of the maximum. Click Reset if you want to play a new game!",
						self.score, self.possible_score, percentage
					))
				} else {
					return String::from("There are no more questions to be chosen from. You can either add more question packs from the game settings and click 'Next question', or return the questions you already went through by clicking 'Reset and next question'.");
				}
			}
			Question::DECQuestion { .. } => {
				return String::from("What is the declination of this point?");
			}
			Question::RAQuestion { .. } => {
				return String::from("What is the right ascension of this point?");
			}
		}
	}

	pub fn should_display_input(&self) -> bool {
		match &self.question_catalog[self.current_question] {
			Question::ObjectQuestion { .. } | Question::NoMoreQuestions => false,
			Question::PositionQuestion { .. } | Question::ThisPointObject { .. } | Question::DistanceBetweenQuestion { .. } | Question::DECQuestion { .. } | Question::RAQuestion { .. } => true,
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
			| Question::RAQuestion { .. } => false,
		}
	}

	pub fn reset_used_questions(&mut self, cellestial_sphere: &mut CellestialSphere) {
		self.used_questions = Vec::new();
		self.score = 0;
		self.possible_score = 0;
		let old_catalog = self.question_catalog.to_vec();
		self.question_catalog = old_catalog
			.into_iter()
			.map(|x| {
				match x {
					Question::NoMoreQuestions | Question::ObjectQuestion { .. } | Question::ThisPointObject { .. } => return x,
					Question::DECQuestion { .. } => {
						let (ra, dec) = geometry::generate_random_point(&mut rand::thread_rng());
						return Question::DECQuestion { ra, dec };
					}
					Question::RAQuestion { .. } => {
						let (ra, dec) = geometry::generate_random_point(&mut rand::thread_rng());
						return Question::RAQuestion { ra, dec };
					}
					Question::PositionQuestion { .. } => {
						let (ra, dec) = geometry::generate_random_point(&mut rand::thread_rng());
						let abbrev = cellestial_sphere.determine_constellation((ra, dec));
						let possible_constellation_names: Vec<String> = match cellestial_sphere.constellations.get(&abbrev) {
							None => vec![String::from("Undefined")],
							Some(constellation) => constellation.possible_names.to_owned(),
						};
						return Question::PositionQuestion {
							ra,
							dec,
							possible_constellation_names,
						};
					}
					Question::DistanceBetweenQuestion { .. } => {
						return Question::DistanceBetweenQuestion {
							point1: geometry::generate_random_point(&mut rand::thread_rng()),
							point2: geometry::generate_random_point(&mut rand::thread_rng()),
						};
					}
				};
			})
			.collect();
	}
	pub fn show_circle_marker(&self) -> bool {
		match &self.question_catalog[self.current_question] {
			Question::NoMoreQuestions | Question::PositionQuestion { .. } | Question::DistanceBetweenQuestion { .. } | Question::DECQuestion { .. } | Question::RAQuestion { .. } => false,
			Question::ObjectQuestion { is_bayer, is_starname, .. } | Question::ThisPointObject { is_bayer, is_starname, .. } => {
				if *is_bayer || *is_starname {
					true
				} else {
					false
				}
			}
		}
	}
}
