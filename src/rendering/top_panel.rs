use eframe::egui;

use crate::Application;

impl Application {
	pub fn render_top_panel(&mut self, ctx: &egui::Context) {
		egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
			egui::menu::bar(ui, |ui| {
				ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
					let app_info_btn = ui
						.add(egui::Button::new(
							egui::RichText::new("App info").text_style(egui::TextStyle::Body),
						))
						.on_hover_text("Show information about the application");
						if app_info_btn.clicked() {
							self.state.windows.app_info.opened = true;
						}
					let stats_btn = ui
						.add(egui::Button::new(
							egui::RichText::new("Statistics").text_style(egui::TextStyle::Body),
						))
						.on_hover_text("Show your statistics");
					if stats_btn.clicked() {
						self.state.windows.stats.opened = true;
					}
				});
			});
		});
	}
}