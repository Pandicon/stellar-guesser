use eframe::egui;

use crate::Application;

mod app_info_window;
mod stats_window;
mod top_panel;

impl Application {
	pub fn render(&mut self, ctx: &egui::Context) {
		self.render_application_info_window(ctx);
		self.render_statistics_window(ctx);
		egui::CentralPanel::default().show(ctx, |ui| {
			self.render_top_panel(ctx);
			let painter = ui.painter();
			self.cellestial_sphere.render_sky(painter, ctx);
		});
	}
}