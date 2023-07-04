use eframe::egui;

use crate::Application;

impl Application {
	pub fn render_game_settings_window(&mut self, ctx: &egui::Context) -> Option<egui::InnerResponse<Option<()>>> {
		egui::Window::new("Game settings").open(&mut self.state.windows.game_settings.opened).show(ctx, |ui| {
			ui.label("AAAH");
		})
	}
}
