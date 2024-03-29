use crate::Application;

impl Application {
	pub fn render_statistics_window(&mut self, ctx: &egui::Context) -> Option<egui::InnerResponse<Option<()>>> {
		egui::Window::new("Application info").open(&mut self.state.windows.app_info.opened).show(ctx, |ui| {
			ui.label(format!("Authors: {}", self.authors));
			ui.label(format!("Version: {}", self.version));
		})
	}
}
