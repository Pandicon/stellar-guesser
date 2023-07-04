use eframe::egui;

use crate::Application;

impl Application {
	pub fn render_question_window(&mut self, ctx: &egui::Context) -> Option<egui::InnerResponse<Option<()>>> {
		egui::Window::new("Question").open(&mut self.state.windows.game_question.opened).show(ctx, |ui| {
			ui.heading(self.game_handler.get_display_question());
			if ui.button("Done").clicked() {
				self.game_handler.next_question();
			}
		})
	}
}
