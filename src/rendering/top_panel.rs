use std::f32::consts::PI;

use eframe::egui;

use crate::{enums::LightPollution, Application};

impl Application {
	pub fn render_top_panel(&mut self, ctx: &egui::Context) -> egui::InnerResponse<()> {
		egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
			egui::menu::bar(ui, |ui| {
				ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
					ui.label(format!("FOV: {}°", 4.0 * (1.0 / self.cellestial_sphere.get_zoom()).atan() / PI * 180.0));
					ui.label(self.frames_handler.fps_display_holder.clone());
					ui.label(self.frames_handler.average_fps_display_holder.clone()).on_hover_text(format!(
						"The average FPS over the last {} frame{}",
						self.frames_handler.frames_analysed,
						if self.frames_handler.frames_analysed != 1 { "s" } else { "" }
					));
					let prev_light_pollution: LightPollution = self.cellestial_sphere.light_pollution_place;
					ui.label("Light pollution level: ");
					egui::ComboBox::from_id_source("Light pollution level: ")
						.selected_text(format!("{}", self.cellestial_sphere.light_pollution_place))
						.show_ui(ui, |ui: &mut egui::Ui| {
							ui.style_mut().wrap = Some(false);
							ui.selectable_value(&mut self.cellestial_sphere.light_pollution_place, LightPollution::Default, format!("{}", LightPollution::Default));
							ui.selectable_value(&mut self.cellestial_sphere.light_pollution_place, LightPollution::Prague, format!("{}", LightPollution::Prague));
							ui.selectable_value(&mut self.cellestial_sphere.light_pollution_place, LightPollution::AverageVillage, format!("{}",LightPollution::AverageVillage))
						});
					if prev_light_pollution != self.cellestial_sphere.light_pollution_place {
						let [mag_offset, mag_scale] = self.cellestial_sphere.light_pollution_place_to_mag_settings(&self.cellestial_sphere.light_pollution_place);
						self.cellestial_sphere.mag_offset = mag_offset;
						self.cellestial_sphere.mag_scale = mag_scale;
					}
				});
				ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
					let app_info_btn = ui
						.add(egui::Button::new(egui::RichText::new("App info").text_style(egui::TextStyle::Body)))
						.on_hover_text("Show information about the application");
					if app_info_btn.clicked() {
						self.state.windows.app_info.opened = true;
					}
					let stats_btn: egui::Response = ui
						.add(egui::Button::new(egui::RichText::new("Statistics").text_style(egui::TextStyle::Body)))
						.on_hover_text("Show your statistics");
					if stats_btn.clicked() {
						self.state.windows.stats.opened = true;
					}
					let graphics_settings_btn = ui
						.add(egui::Button::new(egui::RichText::new("Sky settings").text_style(egui::TextStyle::Body)))
						.on_hover_text("Show the sky settings");
					if graphics_settings_btn.clicked() {
						self.state.windows.graphics_settings.opened = true;
					}
					let game_settings_btn = ui
						.add(egui::Button::new(egui::RichText::new("Game settings").text_style(egui::TextStyle::Body)))
						.on_hover_text("Show the game settings");
					if game_settings_btn.clicked() {
						self.state.windows.game_settings.opened = true;
					}
				let game_question_btn = ui
				.add(egui::Button::new(egui::RichText::new("Question").text_style(egui::TextStyle::Body)))
				.on_hover_text("Show the question");
				if game_question_btn.clicked() {
					self.state.windows.game_question.opened = true;
				}
				});
			});
		})
	}
}
