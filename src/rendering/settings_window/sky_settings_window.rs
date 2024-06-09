use std::collections::HashSet;

use egui::epaint::Color32;

use crate::{
    enums::{ColourMode, LightPollution},
    renderer::CellestialSphere,
    structs::state::windows::settings::SkySettingsSubWindow,
    Application,
};

impl Application {
    pub fn render_sky_settings_window(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(
                &mut self.state.windows.settings.sky_settings.subwindow,
                SkySettingsSubWindow::General,
                SkySettingsSubWindow::General.as_ref(),
            );
            ui.selectable_value(
                &mut self.state.windows.settings.sky_settings.subwindow,
                SkySettingsSubWindow::Stars,
                SkySettingsSubWindow::Stars.as_ref(),
            );
            ui.selectable_value(
                &mut self.state.windows.settings.sky_settings.subwindow,
                SkySettingsSubWindow::Deepsky,
                SkySettingsSubWindow::Deepsky.as_ref(),
            );
            ui.selectable_value(
                &mut self.state.windows.settings.sky_settings.subwindow,
                SkySettingsSubWindow::Lines,
                SkySettingsSubWindow::Lines.as_ref(),
            );
            ui.selectable_value(
                &mut self.state.windows.settings.sky_settings.subwindow,
                SkySettingsSubWindow::Markers,
                SkySettingsSubWindow::Markers.as_ref(),
            );
        });
        ui.separator();
        match self.state.windows.settings.sky_settings.subwindow {
            SkySettingsSubWindow::General => self.render_sky_settings_general_subwindow(ctx, ui),
            SkySettingsSubWindow::Stars => self.render_sky_settings_stars_subwindow(ui),
            SkySettingsSubWindow::Deepsky => self.render_sky_settings_deepsky_subwindow(ui),
            SkySettingsSubWindow::Lines => self.render_sky_settings_lines_subwindow(ui),
            SkySettingsSubWindow::Markers => self.render_sky_settings_markers_subwindow(ui),
        }
    }

    pub fn render_sky_settings_general_subwindow(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
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
            log::error!("{}", serde_json::to_string_pretty(&ctx.style().visuals).unwrap());
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
    }

    pub fn render_sky_settings_stars_subwindow(&mut self, ui: &mut egui::Ui) {
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
    }

    pub fn render_sky_settings_deepsky_subwindow(&mut self, ui: &mut egui::Ui) {
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
    }

    pub fn render_sky_settings_lines_subwindow(&mut self, ui: &mut egui::Ui) {
        let mut line_groups_to_init = HashSet::new();
        let mut line_groups_to_deinit = HashSet::new();
        for (name, lines_set) in &mut self.cellestial_sphere.lines {
            ui.heading(name);
            if ui.checkbox(&mut lines_set.active, format!("Render lines from the {} file", name)).changed() {
                if lines_set.active {
                    line_groups_to_init.insert(name.to_owned());
                } else {
                    line_groups_to_deinit.insert(name.to_owned());
                }
                self.cellestial_sphere.sky_settings.lines_categories_active.insert(name.to_owned(), lines_set.active);
            }
            let mut colour = lines_set.colour.to_srgba_unmultiplied().map(|n| (n as f32) / 255.0);
            ui.horizontal(|ui| {
                ui.label("Line colour: ");
                if ui.color_edit_button_rgba_unmultiplied(&mut colour).changed() {
                    line_groups_to_init.insert(name.to_owned());
                }
            });
            let colour = colour.map(|n| (n * 255.0) as u8);
            lines_set.colour = Color32::from_rgba_unmultiplied(colour[0], colour[1], colour[2], colour[3]);
        }
        for name in &line_groups_to_init {
            self.cellestial_sphere.init_single_renderer("lines", name);
        }
        for name in &line_groups_to_deinit {
            self.cellestial_sphere.deinit_single_renderer("lines", name);
        }
    }

    pub fn render_sky_settings_markers_subwindow(&mut self, ui: &mut egui::Ui) {
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
    }
}
