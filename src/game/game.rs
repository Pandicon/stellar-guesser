use std::f32::consts::PI;

use crate::{caspr::CellestialSphere, markers::Marker};
use eframe::epaint::Color32;
use rand::Rng;

pub enum Question {
	ObjectQuestion { name: String, ra: f32, dec: f32 },
	PositionQuestion { ra: f32, dec: f32 },
	ThisPointObject { possible_names: Vec<String>, ra: f32, dec: f32 },
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
}

impl GameHandler {
	pub fn init(cellestial_sphere: &mut CellestialSphere) -> Self {
		let mut catalog: Vec<Question> = Vec::new();
		for file in cellestial_sphere.deepskies.values() {
			for deepsky in file {
				let mut possible_names = Vec::new();
				if let Some(messier_name) = &deepsky.messier {
					catalog.push(Question::ObjectQuestion {
						name: messier_name.to_owned(),
						ra: deepsky.ra,
						dec: deepsky.dec,
					});
					possible_names.push(messier_name.to_owned());
				}
				if let Some(caldwell_number) = &deepsky.caldwell {
					let caldwell_name = format!("C {}", caldwell_number);
					catalog.push(Question::ObjectQuestion {
						name: caldwell_name.to_owned(),
						ra: deepsky.ra,
						dec: deepsky.dec,
					});
					possible_names.push(caldwell_name.to_owned());
				}
				if let Some(ngc_number) = &deepsky.ngc {
					let ngc_name = format!("NGC {}", ngc_number);
					catalog.push(Question::ObjectQuestion {
						name: ngc_name.to_owned(),
						ra: deepsky.ra,
						dec: deepsky.dec,
					});
					possible_names.push(ngc_name.to_owned());
				}
				if let Some(ic_number) = &deepsky.ic {
					let ic_name = format!("IC {}", ic_number);
					catalog.push(Question::ObjectQuestion {
						name: ic_name.to_owned(),
						ra: deepsky.ra,
						dec: deepsky.dec,
					});
					possible_names.push(ic_name.to_owned());
				}
				if !possible_names.is_empty() {
					catalog.push(Question::ThisPointObject {
						possible_names,
						ra: deepsky.ra,
						dec: deepsky.dec,
					});
				}
			}
		}

		let current_question = rand::thread_rng().gen_range(0..catalog.len());

		let entry = cellestial_sphere.markers.entry("game".to_string()).or_default();
		let add_marker_on_click = match catalog[current_question] {
			Question::ObjectQuestion { .. } => {
				*entry = Vec::new();
				true
			}
			Question::PositionQuestion { ra, dec, .. } | Question::ThisPointObject { ra, dec, .. } => {
				*entry = vec![Marker::new(ra, dec, Color32::YELLOW, 2.0, 5.0, false, false)];
				false
			}
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
		}
	}

	pub fn check_answer(&mut self, cellestial_sphere: &mut crate::caspr::CellestialSphere) {
		self.stage = 1;
		match &self.questions[self.current_question] {
			Question::ObjectQuestion { .. } => {
				self.answer_review_text_heading = format!("");
				self.answer_review_text = String::from("To be implemented");
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
		}
	}

	pub fn next_question(&mut self, cellestial_sphere: &mut crate::caspr::CellestialSphere) {
		self.answer = String::new();
		self.used_questions.push(self.current_question);
		let mut possible_questions: Vec<usize> = Vec::new();
		for question in 0..self.questions.len() {
			if !self.used_questions.contains(&question) {
				possible_questions.push(question);
			}
		}
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
		};
		self.stage = 0;
	}
	pub fn get_display_question(&self) -> String {
		match &self.questions[self.current_question] {
			Question::ObjectQuestion { name, ra: _ra, dec: _dec } => {
				return String::from(format!("Find {}.", name));
			}
			Question::PositionQuestion { ra: _ra, dec: _dec } => {
				return String::from("This does not work yet... Sorry :)");
			}
			Question::ThisPointObject {
				possible_names: _possible_names,
				ra: _ra,
				dec: _dec,
			} => {
				return String::from("What is this object?");
			}
		}
	}

	pub fn should_display_input(&self) -> bool {
		match &self.questions[self.current_question] {
			Question::ObjectQuestion { .. } => false,
			Question::PositionQuestion { .. } | Question::ThisPointObject { .. } => true,
		}
	}
}
