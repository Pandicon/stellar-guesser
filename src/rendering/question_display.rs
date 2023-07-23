use eframe::egui;

use crate::Application;

impl Application {
	pub fn render_question_window(&mut self, ctx: &egui::Context) -> Option<egui::InnerResponse<Option<()>>> {
		egui::Window::new("Question").open(&mut self.state.windows.game_question.opened).show(ctx, |ui| {
			if self.game_handler.stage == 0 {
				if self.game_handler.no_more_questions() {
					if self.game_handler.is_scored_mode {
						ui.heading("Game over!");
					} else {
						ui.heading("No more questions left");
					}
					ui.label(self.game_handler.get_display_question());
					ui.horizontal(|ui| {
						if ui.button("Next question").clicked() {
							self.game_handler.next_question(&mut self.cellestial_sphere);
						}
						if ui.button("Reset").clicked() {
							self.game_handler.reset_used_questions(&mut self.cellestial_sphere);
							self.game_handler.next_question(&mut self.cellestial_sphere);
						}
					});
				} else {
					ui.heading(self.game_handler.get_display_question());
					if self.game_handler.should_display_input() {
						ui.text_edit_singleline(&mut self.game_handler.answer);
					}
					if ui.button("Check").clicked() {
						self.game_handler.check_answer(&mut self.cellestial_sphere);
					}
				}
				ui.label(&self.game_handler.question_number_text);
			} else if self.game_handler.stage == 1 {
				if !self.game_handler.answer_review_text_heading.is_empty() {
					ui.heading(&self.game_handler.answer_review_text_heading);
				}
				ui.label(&self.game_handler.answer_review_text);
				if ui.button("Next").clicked() {
					self.game_handler.next_question(&mut self.cellestial_sphere);
				}
				ui.label(&self.game_handler.question_number_text);
			} else if self.game_handler.stage == 2 {
				ui.heading("Welcome!");
				if ui.button("Start").clicked() {
					self.game_handler.stage = 1;
					self.game_handler.next_question(&mut self.cellestial_sphere)
				}
			} else {
				unimplemented!();
			}
		})
	}
}
