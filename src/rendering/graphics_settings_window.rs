use eframe::{egui, epaint::Color32};

use crate::Application;

impl Application {
	pub fn render_graphics_settings_window(&mut self, ctx: &egui::Context) -> Option<egui::InnerResponse<Option<()>>> {
		egui::Window::new("Graphics settings").open(&mut self.state.windows.graphics_settings.opened).show(ctx, |ui| {
			egui::CollapsingHeader::new(egui::RichText::new("Stars").text_style(egui::TextStyle::Heading).size(20.0))
				.default_open(true)
				.show(ui, |ui| {
					ui.checkbox(&mut self.graphics_settings.use_default_star_colour, "Use default star colour");
					let mut colour = [
						(self.graphics_settings.default_star_colour.r() as f32) / 255.0,
						(self.graphics_settings.default_star_colour.g() as f32) / 255.0,
						(self.graphics_settings.default_star_colour.b() as f32) / 255.0,
						(self.graphics_settings.default_star_colour.a() as f32) / 255.0,
					];
					ui.horizontal(|ui| {
						ui.color_edit_button_rgba_premultiplied(&mut colour);
						ui.label("Default star colour");
					});
					self.graphics_settings.default_star_colour =
						Color32::from_rgba_premultiplied((colour[0] * 255.0) as u8, (colour[1] * 255.0) as u8, (colour[2] * 255.0) as u8, (colour[3] * 255.0) as u8);
				});
		})
	}
}
