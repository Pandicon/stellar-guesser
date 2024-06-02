use egui::epaint::Color32;

use crate::{
	caspr::CellestialSphere,
	enums::{ColourMode, LightPollution},
	Application,
};

impl Application {
	pub fn render_sky_settings_window(&mut self, ctx: &egui::Context) -> Option<egui::InnerResponse<Option<()>>> {
		egui::Window::new("Sky settings").open(&mut self.state.windows.graphics_settings.opened).show(ctx, |ui| {
			let prev_light_pollution = self.cellestial_sphere.light_pollution_place;
			ui.label("Light pollution level: ");
			egui::ComboBox::from_id_source("Light pollution level: ")
				.selected_text(format!("{}", self.cellestial_sphere.light_pollution_place))
				.show_ui(ui, |ui| {
					ui.style_mut().wrap = Some(false);
					ui.selectable_value(&mut self.cellestial_sphere.light_pollution_place, LightPollution::Default, format!("{}", LightPollution::Default));
					ui.selectable_value(&mut self.cellestial_sphere.light_pollution_place, LightPollution::Prague, format!("{}", LightPollution::Prague));
					ui.selectable_value(
						&mut self.cellestial_sphere.light_pollution_place,
						LightPollution::AverageVillage,
						format!("{}", LightPollution::AverageVillage),
					);
				});
			if prev_light_pollution != self.cellestial_sphere.light_pollution_place {
				let [mag_offset, mag_scale] = self.cellestial_sphere.light_pollution_place_to_mag_settings(&self.cellestial_sphere.light_pollution_place);
				self.cellestial_sphere.sky_settings.mag_offset = mag_offset;
				self.cellestial_sphere.sky_settings.mag_scale = mag_scale;
			}
			egui::CollapsingHeader::new(egui::RichText::new("Stars").text_style(egui::TextStyle::Heading).size(20.0))
				.default_open(true)
				.show(ui, |ui| {
					let colour_mode = self.graphics_settings.colour_mode;
					ui.label("Colour mode: ");
					egui::ComboBox::from_id_source("Colour mode: ")
						.selected_text(format!("{}", self.graphics_settings.colour_mode))
						.show_ui(ui, |ui| {
							ui.style_mut().wrap = Some(false);
							ui.selectable_value(&mut self.graphics_settings.colour_mode, ColourMode::Dark, format!("{}", ColourMode::Dark));
							ui.selectable_value(&mut self.graphics_settings.colour_mode, ColourMode::Light, format!("{}", ColourMode::Light));
							ui.selectable_value(&mut self.graphics_settings.colour_mode, ColourMode::Printing, format!("{}", ColourMode::Printing));
						});
					if self.graphics_settings.colour_mode != colour_mode {
						match self.graphics_settings.colour_mode {
							ColourMode::Dark => ctx.set_visuals(egui::Visuals::dark()),
							ColourMode::Light => ctx.set_visuals(egui::Visuals::light()),
							ColourMode::Printing => {
								let mut visuals = egui::Visuals::light();
								visuals.panel_fill = Color32::WHITE;
								visuals.window_fill = Color32::WHITE;
								ctx.set_visuals(visuals)
							}
						}
					}
					ui.checkbox(&mut self.graphics_settings.use_default_star_colour, "Use default star colour");
					let default_star_colour = self.graphics_settings.default_star_colour(&self.graphics_settings.colour_mode);
					let mut colour = [
						(default_star_colour.r() as f32) / 255.0,
						(default_star_colour.g() as f32) / 255.0,
						(default_star_colour.b() as f32) / 255.0,
						(default_star_colour.a() as f32) / 255.0,
					];
					ui.horizontal(|ui| {
						ui.color_edit_button_rgba_premultiplied(&mut colour);
						ui.label("Default star colour");
					});

					let prev_mag_offset = self.cellestial_sphere.sky_settings.mag_offset;
					let prev_mag_scale = self.cellestial_sphere.sky_settings.mag_scale;
					ui.horizontal_wrapped(|ui| ui.label("The following two values affect the size of the stars via the following formula: radius = mag_scale * (mag_offset - magnitude)"));
					ui.horizontal(|ui| {
						self.input.input_field_has_focus |= ui.add(egui::DragValue::new(&mut self.cellestial_sphere.sky_settings.mag_offset).speed(0.1)).has_focus();
						ui.label("Magnitude offset (mag_offset)");
					});
					ui.horizontal(|ui| {
						self.input.input_field_has_focus |= ui.add(egui::DragValue::new(&mut self.cellestial_sphere.sky_settings.mag_scale).speed(0.1)).has_focus();
						ui.label("Magnitude scale (mag_scale)");
					});
					if prev_mag_offset != self.cellestial_sphere.sky_settings.mag_offset || prev_mag_scale != self.cellestial_sphere.sky_settings.mag_scale {
						self.cellestial_sphere.light_pollution_place = CellestialSphere::mag_settings_to_light_pollution_place(
							self.cellestial_sphere.sky_settings.mag_offset,
							self.cellestial_sphere.sky_settings.mag_scale,
							&self.cellestial_sphere.light_pollution_place_to_mag,
						);
					}

					match self.graphics_settings.colour_mode {
						ColourMode::Dark => {
							self.graphics_settings.default_star_colour_dark_mode =
								Color32::from_rgba_premultiplied((colour[0] * 255.0) as u8, (colour[1] * 255.0) as u8, (colour[2] * 255.0) as u8, (colour[3] * 255.0) as u8);
						}
						ColourMode::Light => {
							self.graphics_settings.default_star_colour_light_mode =
								Color32::from_rgba_premultiplied((colour[0] * 255.0) as u8, (colour[1] * 255.0) as u8, (colour[2] * 255.0) as u8, (colour[3] * 255.0) as u8);
						}
						ColourMode::Printing => {
							self.graphics_settings.default_star_colour_print_mode =
								Color32::from_rgba_premultiplied((colour[0] * 255.0) as u8, (colour[1] * 255.0) as u8, (colour[2] * 255.0) as u8, (colour[3] * 255.0) as u8);
						}
					}

					let mut newly_active_star_groups = Vec::new();
					let mut newly_inactive_star_groups = Vec::new();
					for (name, active) in &mut self.cellestial_sphere.sky_settings.stars_categories_active {
						let active_before = *active;
						ui.checkbox(active, format!("Render stars from the {} file", name));
						if !active_before && *active {
							newly_active_star_groups.push(name.to_owned());
						} else if active_before && !*active {
							newly_inactive_star_groups.push(name.to_owned());
						}
					}

					for name in &newly_active_star_groups {
						self.cellestial_sphere.init_single_renderer("stars", name);
					}
					for name in &newly_inactive_star_groups {
						self.cellestial_sphere.deinit_single_renderer("stars", name);
					}
				});
			egui::CollapsingHeader::new(egui::RichText::new("Deepsky objects").text_style(egui::TextStyle::Heading).size(20.0))
				.default_open(true)
				.show(ui, |ui| {
					ui.horizontal(|ui| {
						self.input.input_field_has_focus |= ui
							.add(egui::DragValue::new(&mut self.cellestial_sphere.sky_settings.deepsky_render_mag_decrease).speed(0.1))
							.has_focus();
						ui.label("Magnitude decrease")
							.on_hover_text("By how much should the magnitude of the deepsky objects be decreased for rendering - this way the objects can be made to be seen even without zooming in");
					});
					let mut newly_active_deepsky_groups = Vec::new();
					let mut newly_inactive_deepsky_groups = Vec::new();
					for (name, active) in &mut self.cellestial_sphere.sky_settings.deepskies_categories_active {
						let active_before = *active;
						ui.checkbox(active, format!("Render deepsky objects from the {} file", name));
						if !active_before && *active {
							newly_active_deepsky_groups.push(name.to_owned());
						} else if active_before && !*active {
							newly_inactive_deepsky_groups.push(name.to_owned());
						}
					}
					for name in &newly_active_deepsky_groups {
						self.cellestial_sphere.init_single_renderer("deepskies", name);
					}
					for name in &newly_inactive_deepsky_groups {
						self.cellestial_sphere.deinit_single_renderer("deepskies", name);
					}
				});
			egui::CollapsingHeader::new(egui::RichText::new("Lines").text_style(egui::TextStyle::Heading).size(20.0))
				.default_open(true)
				.show(ui, |ui| {
					let mut newly_active_line_groups = Vec::new();
					let mut newly_inactive_line_groups = Vec::new();
					for (name, active) in &mut self.cellestial_sphere.sky_settings.lines_categories_active {
						let active_before = *active;
						ui.checkbox(active, format!("Render lines from the {} file", name));
						if !active_before && *active {
							newly_active_line_groups.push(name.to_owned());
						} else if active_before && !*active {
							newly_inactive_line_groups.push(name.to_owned());
						}
					}
					for name in &newly_active_line_groups {
						self.cellestial_sphere.init_single_renderer("lines", name);
					}
					for name in &newly_inactive_line_groups {
						self.cellestial_sphere.deinit_single_renderer("lines", name);
					}
				});
			egui::CollapsingHeader::new(egui::RichText::new("Markers").text_style(egui::TextStyle::Heading).size(20.0))
				.default_open(true)
				.show(ui, |ui| {
					let mut newly_active_marker_groups = Vec::new();
					let mut newly_inactive_marker_groups = Vec::new();
					for (name, active) in &mut self.cellestial_sphere.sky_settings.markers_categories_active {
						if name == "game" {
							continue;
						}
						let active_before = *active;
						ui.checkbox(active, format!("Render markers from the {} file", name));
						if !active_before && *active {
							newly_active_marker_groups.push(name.to_owned());
						} else if active_before && !*active {
							newly_inactive_marker_groups.push(name.to_owned());
						}
					}
					for name in &newly_active_marker_groups {
						self.cellestial_sphere.init_single_renderer("markers", name);
					}
					for name in &newly_inactive_marker_groups {
						self.cellestial_sphere.deinit_single_renderer("markers", name);
					}
				});
		})
	}
}
