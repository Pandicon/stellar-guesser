use eframe::egui;

use crate::Application;

mod app_info_window;
mod stats_window;
mod top_panel;

impl Application {
	pub fn render(&mut self, ctx: &egui::Context) {
		let mut cursor_within_subwindow = false;
		let mut window_rectangles = Vec::new();
		if let Some(response) = self.render_application_info_window(ctx) {
			cursor_within_subwindow |= response.response.hover_pos().is_some();
			window_rectangles.push([[response.response.rect.right(), response.response.rect.top()], [response.response.rect.left(), response.response.rect.bottom()]]);
		}
		if let Some(response) = self.render_statistics_window(ctx) {
			cursor_within_subwindow |= response.response.hover_pos().is_some();
			window_rectangles.push([[response.response.rect.right(), response.response.rect.top()], [response.response.rect.left(), response.response.rect.bottom()]]);
		}
		let mut viewport_rect = ctx.input(|i| i.screen_rect());
		egui::CentralPanel::default().show(ctx, |ui| {
			let top_panel_response = self.render_top_panel(ctx);
			viewport_rect.min.y = top_panel_response.response.rect.max.y;
			self.cellestial_sphere.viewport_rect = viewport_rect;

			let painter = ui.painter();
			self.cellestial_sphere.render_sky(painter);
		});
	}
}