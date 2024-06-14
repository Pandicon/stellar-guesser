use std::collections::HashSet;

use egui::epaint::Color32;

use crate::{enums::LightPollution, files, public_constants, renderer::CellestialSphere, structs::state::windows::settings::SkySettingsSubWindow, Application};

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
        egui::ScrollArea::vertical()
            .auto_shrink([false, true])
            .show(ui, |ui| match self.state.windows.settings.sky_settings.subwindow {
                SkySettingsSubWindow::General => self.render_sky_settings_general_subwindow(ctx, ui),
                SkySettingsSubWindow::Stars => self.render_sky_settings_stars_subwindow(ui),
                SkySettingsSubWindow::Deepsky => self.render_sky_settings_deepsky_subwindow(ui),
                SkySettingsSubWindow::Lines => self.render_sky_settings_lines_subwindow(ui),
                SkySettingsSubWindow::Markers => self.render_sky_settings_markers_subwindow(ui),
            });
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
        let previous_theme_name = self.theme.name.clone();
        let mut selected_theme_name = self.theme.name.clone();
        ui.label("Theme: ");
        egui::ComboBox::from_id_source("Theme: ").selected_text(&self.theme.name).show_ui(ui, |ui| {
            ui.style_mut().wrap = Some(false);
            let mut themes = self.themes.themes_names().collect::<Vec<&String>>();
            themes.sort();
            for theme_name in themes {
                ui.selectable_value(&mut selected_theme_name, theme_name.to_owned(), theme_name);
            }
        });
        if selected_theme_name != previous_theme_name {
            match self.themes.get(&selected_theme_name) {
                Some(theme) => {
                    self.apply_theme(ctx, theme.clone());
                }
                None => log::error!("Failed to get the selected theme: {}", selected_theme_name),
            }
        }
        ui.heading("Export theme");
        ui.label("Export the current settings into a theme");
        ui.horizontal(|ui| {
            ui.label("Theme name: ");
            self.input.input_field_has_focus |= ui.text_edit_singleline(&mut self.theme.name).has_focus();
        });
        if ui.button("Export").clicked() {
            if let Some(path) = files::get_dir_opt(public_constants::THEMES_FOLDER) {
                #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
                let save_path_opt: Option<std::path::PathBuf> = {
                    let dialog = rfd::FileDialog::new().add_filter("Theme", &["json"]).set_directory(path);
                    dialog.save_file()
                };
                #[cfg(any(target_os = "android", target_os = "ios"))]
                let save_path_opt: Option<std::path::PathBuf> = {
                    let mut save_path_intermediate = path;
                    save_path_intermediate.push(format!("{}--{}.json", &self.theme.name, chrono::Local::now().timestamp_millis()));
                    Some(save_path_intermediate)
                };
                match save_path_opt {
                    Some(save_path) => match serde_json::to_string_pretty(&self.theme) {
                        Ok(theme_to_save) => {
                            if let Some(dir) = save_path.parent() {
                                if !dir.exists() {
                                    if let Err(err) = std::fs::create_dir_all(dir) {
                                        log::error!("Failed to create the folders for the theme: {err}");
                                    }
                                }
                            } else {
                                log::warn!("No theme folder: {:?}", save_path);
                            }
                            if let Err(err) = std::fs::write(save_path, theme_to_save) {
                                log::error!("Failed to save the theme: {err}");
                            } else {
                                self.themes.insert(self.theme.name.clone(), self.theme.clone());
                            }
                        }
                        Err(err) => log::error!("Failed to serialize the theme: {err}"),
                    },
                    None => log::info!("Theme saving cancelled by the user"),
                }
            }
        }
    }

    pub fn render_sky_settings_stars_subwindow(&mut self, ui: &mut egui::Ui) {
        ui.checkbox(&mut self.graphics_settings.use_default_star_colour, "Use default star colour");
        self.theme.game_visuals.use_default_star_colour = self.graphics_settings.use_default_star_colour;
        let mut default_star_colour = self.theme.game_visuals.default_star_colour.to_srgba_unmultiplied().map(|n| (n as f32) / 255.0);
        ui.horizontal(|ui| {
            ui.color_edit_button_rgba_unmultiplied(&mut default_star_colour);
            ui.label("Default star colour");
        });
        let default_star_colour = default_star_colour.map(|n| (n * 255.0) as u8);
        self.theme.game_visuals.default_star_colour = Color32::from_rgba_unmultiplied(default_star_colour[0], default_star_colour[1], default_star_colour[2], default_star_colour[3]);

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
            ui.horizontal(|ui| {
                ui.label("Line colour: ");
                if ui.color_edit_button_srgba(&mut lines_set.colour).changed() {
                    line_groups_to_init.insert(name.to_owned());
                }
            });
            self.theme.game_visuals.lines_colours.insert(name.clone(), lines_set.colour);
        }
        for name in &line_groups_to_init {
            self.cellestial_sphere.init_single_renderer("lines", name);
        }
        for name in &line_groups_to_deinit {
            self.cellestial_sphere.deinit_single_renderer("lines", name);
        }
    }

    pub fn render_sky_settings_markers_subwindow(&mut self, ui: &mut egui::Ui) {
        let mut marker_groups_to_init = HashSet::new();
        let mut marker_groups_to_deinit = HashSet::new();
        for (name, markers_set) in &mut self.cellestial_sphere.markers {
            ui.heading(name);
            if ui.checkbox(&mut markers_set.active, format!("Render markers from the {} file", name)).changed() {
                if markers_set.active {
                    marker_groups_to_init.insert(name.to_owned());
                } else {
                    marker_groups_to_deinit.insert(name.to_owned());
                }
                self.cellestial_sphere.sky_settings.markers_categories_active.insert(name.to_owned(), markers_set.active);
            }
            ui.horizontal(|ui| {
                ui.label("Marker colour: ");
                if ui.color_edit_button_srgba(&mut markers_set.colour).changed() {
                    marker_groups_to_init.insert(name.to_owned());
                }
            });
            self.theme.game_visuals.markers_colours.insert(name.clone(), markers_set.colour);
        }
        for name in &marker_groups_to_init {
            self.cellestial_sphere.init_single_renderer("markers", name);
        }
        for name in &marker_groups_to_deinit {
            self.cellestial_sphere.deinit_single_renderer("markers", name);
        }
    }
}
