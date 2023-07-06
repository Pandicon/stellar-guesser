use std::f32::consts::PI;

use crate::{caspr::CellestialSphere, markers::Marker};
use eframe::epaint::Color32;
use rand::Rng;

#[path = "../geometry.rs"]
mod geometry;

pub enum Question {
	ObjectQuestion {
		name: String,
		ra: f32,
		dec: f32,
		is_messier: bool,
		is_caldwell: bool,
		is_ngc: bool,
		is_ic: bool,
	},
	PositionQuestion {
		ra: f32,
		dec: f32,
	},
	ThisPointObject {
		possible_names: Vec<String>,
		ra: f32,
		dec: f32,
		is_messier: bool,
		is_caldwell: bool,
		is_ngc: bool,
		is_ic: bool,
	},
	NoMoreQuestions,
}

pub struct QuestionSettings {
	pub show_messiers: bool,
	pub show_caldwells: bool,
	pub show_ngcs: bool,
	pub show_ics: bool,
}

impl Default for QuestionSettings {
	fn default() -> Self {
		Self {
			show_messiers: true,
			show_caldwells: true,
			show_ngcs: true,
			show_ics: true,
		}
	}
}

pub struct GameHandler {
	current_question: usize,
	questions: Vec<Question>,
	used_questions: Vec<usize>,

	pub add_marker_on_click: bool,
	/// 0 = guessing, 1 = checked
	pub stage: usize,

	pub answer_review_text_heading: String,
	pub answer_review_text: String,
	pub answer: String,

	pub object_question_settings: QuestionSettings,
	pub this_point_object_question_settings: QuestionSettings,
	pub show_object_questions: bool,
	pub show_positions_questions: bool,
	pub show_this_point_object_questions: bool,
}

impl GameHandler {
	pub fn init(cellestial_sphere: &mut CellestialSphere) -> Self {
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
					});
					possible_names.push(messier_name.to_owned());
				}
				if let Some(caldwell_number) = &deepsky.caldwell {
					let caldwell_name = format!("C {}", caldwell_number);
					catalog.push(Question::ObjectQuestion {
						name: caldwell_name.to_owned(),
						ra: deepsky.ra,
						dec: deepsky.dec,
						is_messier: false,
						is_caldwell: true,
						is_ngc: false,
						is_ic: false,
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
					});
				}
			}
		}

		let current_question = rand::thread_rng().gen_range(0..catalog.len());

		let entry = cellestial_sphere.markers.entry("game".to_string()).or_default();
		let add_marker_on_click: bool = match catalog[current_question] {
			Question::ObjectQuestion { .. } => {
				*entry = Vec::new();
				true
			}
			Question::PositionQuestion { ra, dec, .. } | Question::ThisPointObject { ra, dec, .. } => {
				*entry = vec![Marker::new(ra, dec, Color32::YELLOW, 2.0, 5.0, false, false)];
				false
			}
			Question::NoMoreQuestions => false,
		};
		Self {
			current_question,
			questions: catalog,
			used_questions: Vec::new(),
			add_marker_on_click,
			stage: 0,
			answer_review_text_heading: String::new(),
			answer_review_text: String::new(),
			answer: String::new(),
			object_question_settings: QuestionSettings::default(),
			this_point_object_question_settings: QuestionSettings::default(),
			show_object_questions: true,
			show_positions_questions: true,
			show_this_point_object_questions: true,
		}
	}

	pub fn check_answer(&mut self, cellestial_sphere: &mut crate::caspr::CellestialSphere) {
		self.stage = 1;
		self.add_marker_on_click = false;
		let entry = cellestial_sphere.markers.entry("game".to_string()).or_default();
		match &self.questions[self.current_question] {
			Question::ObjectQuestion { ra, dec, .. } => {
				let (answer_dec_text, answer_ra_text, distance, answer_review_text_heading) = if !entry.is_empty() {
					let answer_dec = entry[0].dec;
					let answer_ra = entry[0].ra;
					let distance = geometry::angular_distance((ra * PI / 180.0, dec * PI / 180.0), (answer_ra * PI / 180.0, answer_dec * PI / 180.0)) * 180.0 / PI;
					(
						answer_dec.to_string(),
						answer_ra.to_string(),
						distance.to_string(),
						format!("You were {} degrees away!", (distance * 100.0).round() / 100.0),
					)
				} else {
					(String::from("-"), String::from("-"), String::from("-"), String::from("You didn't guess"))
				};
				self.answer_review_text_heading = answer_review_text_heading;
				self.answer_review_text = format!(
					"Your coordinates: [dec = {}; ra = {}]\nCorrect coordinates: [dec = {}; ra = {}]\nFully precise distance: {} degrees\nYou can see the correct place marked with a yellow cross.",
					answer_dec_text, answer_ra_text, dec, ra, distance
				);
				entry.push(Marker::new(*ra, *dec, Color32::YELLOW, 2.0, 5.0, false, false));
			}
			Question::PositionQuestion { ra, dec, .. } => {
				self.answer_review_text_heading = format!("");
				self.answer_review_text = String::from("Not implemented yet D:");
			}
			Question::ThisPointObject { possible_names, .. } => {
				let possible_names_edited = possible_names.iter().map(|name| name.replace(" ", "").to_lowercase()).collect::<Vec<String>>();
				let correct = possible_names_edited.contains(&self.answer.replace(" ", "").to_lowercase());
				self.answer_review_text_heading = format!("{}orrect!", if correct { "C" } else { "Inc" });
				self.answer_review_text = format!("Your answer was: {}\nPossible answers: {}", self.answer, possible_names.join(", "));
			}
			Question::NoMoreQuestions => {}
		}
	}

	pub fn next_question(&mut self, cellestial_sphere: &mut crate::caspr::CellestialSphere) {
		self.answer = String::new();
		self.used_questions.push(self.current_question);
		let mut possible_questions: Vec<usize> = Vec::new();
		for question in 0..self.questions.len() {
			if !self.used_questions.contains(&question) {
				match self.questions[question] {
					Question::ObjectQuestion {
						is_messier,
						is_caldwell,
						is_ngc,
						is_ic,
						..
					} => {
						if self.show_object_questions
							&& ((self.object_question_settings.show_messiers && is_messier)
								|| (self.object_question_settings.show_caldwells && is_caldwell)
								|| (self.object_question_settings.show_ngcs && is_ngc)
								|| (self.object_question_settings.show_ics && is_ic))
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
						..
					} => {
						if self.show_this_point_object_questions
							&& ((self.this_point_object_question_settings.show_messiers && is_messier)
								|| (self.this_point_object_question_settings.show_caldwells && is_caldwell)
								|| (self.this_point_object_question_settings.show_ngcs && is_ngc)
								|| (self.this_point_object_question_settings.show_ics && is_ic))
						{
							possible_questions.push(question);
						}
					}
					Question::NoMoreQuestions => {}
				}
			}
		}
		if possible_questions.is_empty() {
			self.current_question = 0;
		} else {
			self.current_question = possible_questions[rand::thread_rng().gen_range(0..possible_questions.len())];

			let entry = cellestial_sphere.markers.entry("game".to_string()).or_default();
			self.add_marker_on_click = match self.questions[self.current_question] {
				Question::ObjectQuestion { .. } => {
					*entry = Vec::new();
					true
				}
				Question::PositionQuestion { ra, dec, .. } | Question::ThisPointObject { ra, dec, .. } => {
					*entry = vec![Marker::new(ra, dec, Color32::YELLOW, 2.0, 5.0, false, false)];
					false
				}
				Question::NoMoreQuestions => false,
			};
		}
		self.stage = 0;
	}
	pub fn get_display_question(&self) -> String {
		match &self.questions[self.current_question] {
			Question::ObjectQuestion { name, .. } => {
				return String::from(format!("Find {}.", name));
			}
			Question::PositionQuestion { ra: _ra, dec: _dec } => {
				return String::from("This does not work yet... Sorry :)");
			}
			Question::ThisPointObject { .. } => {
				return String::from("What is this object?");
			}
			Question::NoMoreQuestions => {
				return String::from("There are no more questions to be chosen from. You can either add more question packs from the game settings and click 'Next question', or return the questions you already went through by clicking 'Reset and next question'.");
			}
		}
	}

	pub fn should_display_input(&self) -> bool {
		match &self.questions[self.current_question] {
			Question::ObjectQuestion { .. } | Question::NoMoreQuestions => false,
			Question::PositionQuestion { .. } | Question::ThisPointObject { .. } => true,
		}
	}

	pub fn no_more_questions(&self) -> bool {
		match &self.questions[self.current_question] {
			Question::NoMoreQuestions => true,
			Question::ObjectQuestion { .. } | Question::PositionQuestion { .. } | Question::ThisPointObject { .. } => false,
		}
	}

	pub fn reset_used_questions(&mut self) {
		self.used_questions = Vec::new();
	}
}
