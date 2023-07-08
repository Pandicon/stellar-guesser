use eframe::egui;

use crate::Application;

impl Application {
	pub fn render_game_settings_window(&mut self, ctx: &egui::Context) -> Option<egui::InnerResponse<Option<()>>> {
		egui::Window::new("Game settings").open(&mut self.state.windows.game_settings.opened).show(ctx, |ui| {
			egui::CollapsingHeader::new(egui::RichText::new("'Find this object' questions").text_style(egui::TextStyle::Heading).size(20.0))
				.default_open(true)
				.show(ui, |ui| {
					ui.checkbox(&mut self.game_handler.show_object_questions, "Show the 'Find this object' questions");
					ui.checkbox(&mut self.game_handler.object_question_settings.show_messiers, "Show Messier objects");
					ui.checkbox(&mut self.game_handler.object_question_settings.show_caldwells, "Show Caldwell objects");
					ui.checkbox(&mut self.game_handler.object_question_settings.show_ngcs, "Show NGC objects");
					ui.checkbox(&mut self.game_handler.object_question_settings.show_ics, "Show IC objects");
				});
			egui::CollapsingHeader::new(egui::RichText::new("'Which constellation is this point in' questions").text_style(egui::TextStyle::Heading).size(20.0))
				.default_open(true)
				.show(ui, |ui| {
					ui.checkbox(&mut self.game_handler.show_positions_questions, "Show the 'Which constellation is this point in' questions");
				});
			egui::CollapsingHeader::new(egui::RichText::new("'What is this object' questions").text_style(egui::TextStyle::Heading).size(20.0))
				.default_open(true)
				.show(ui, |ui| {
					ui.checkbox(&mut self.game_handler.show_this_point_object_questions, "Show the 'What is this object' questions");
					ui.checkbox(&mut self.game_handler.this_point_object_question_settings.show_messiers, "Show Messier objects");
					ui.checkbox(&mut self.game_handler.this_point_object_question_settings.show_caldwells, "Show Caldwell objects");
					ui.checkbox(&mut self.game_handler.this_point_object_question_settings.show_ngcs, "Show NGC objects");
					ui.checkbox(&mut self.game_handler.this_point_object_question_settings.show_ics, "Show IC objects");
				});
			egui::CollapsingHeader::new(egui::RichText::new("'What is the angle between..' questions").text_style(egui::TextStyle::Heading).size(20.0))
				.default_open(true)
				.show(ui, |ui| {
					ui.checkbox(&mut self.game_handler.show_distance_between_questions, "Show the 'What is the angle between..' questions");
				});
			egui::CollapsingHeader::new(egui::RichText::new("'What is the RA/DEC..' questions").text_style(egui::TextStyle::Heading).size(20.0))
				.default_open(true)
				.show(ui, |ui: &mut egui::Ui| {
					ui.checkbox(&mut self.game_handler.show_radecquestions, "Show the 'What is the RA/DEC..' questions");
				});
			ui.checkbox(&mut self.game_handler.is_scored_mode, "Play in scored mode?");
			ui.add(
				egui::Slider::new(&mut self.game_handler.no_of_questions, 1..=self.game_handler.possible_no_of_questions)
					.text("Number of questions")
					.logarithmic(true),
			);
			if ui.button("Reset").clicked() {
				self.game_handler.stage = 2;
				self.game_handler.reset_used_questions();
			}
		})
	}
}
