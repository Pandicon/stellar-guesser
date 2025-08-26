use std::collections::HashSet;

use eframe::egui;

use crate::{
    enums::{LightPollution, RendererCategory},
    renderer::CellestialSphere,
    rendering::caspr::{markers::game_markers::GameMarker, stars},
    structs::state::windows::settings::SkySettingsSubWindow,
    Application,
};

impl Application {
    pub fn render_sky_settings_window(&mut self, ui: &mut egui::Ui) {
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
                SkySettingsSubWindow::General => self.render_sky_settings_general_subwindow(ui),
                SkySettingsSubWindow::Stars => self.render_sky_settings_stars_subwindow(ui),
                SkySettingsSubWindow::Deepsky => self.render_sky_settings_deepsky_subwindow(ui),
                SkySettingsSubWindow::Lines => self.render_sky_settings_lines_subwindow(ui),
                SkySettingsSubWindow::Markers => self.render_sky_settings_markers_subwindow(ui),
            });
    }

    pub fn render_sky_settings_general_subwindow(&mut self, ui: &mut egui::Ui) {
        let prev_light_pollution = self.cellestial_sphere.light_pollution_place;
        ui.label("Light pollution level: ")
            .on_hover_text("These settings are made to reflect how the sky looks in different locations for a person with an average eyesight.");
        egui::ComboBox::from_id_salt("Light pollution level: ")
            .selected_text(format!("{}", self.cellestial_sphere.light_pollution_place))
            .show_ui(ui, |ui| {
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                for val in LightPollution::variants() {
                    ui.selectable_value(&mut self.cellestial_sphere.light_pollution_place, val, format!("{val}"))
                        .on_hover_text(LightPollution::explanation(&val));
                }
            });
        if prev_light_pollution != self.cellestial_sphere.light_pollution_place {
            let settings = self.cellestial_sphere.light_pollution_place_to_mag_settings(&self.cellestial_sphere.light_pollution_place);
            self.cellestial_sphere.sky_settings.mag_to_radius_settings[self.cellestial_sphere.sky_settings.mag_to_radius_id] = settings;
        }
        ui.separator();
        let previous_enabled = self.cellestial_sphere.sky_settings.cloud_settings.enabled;
        let previous_coverage = self.cellestial_sphere.sky_settings.cloud_settings.coverage;
        let previous_thickness = self.cellestial_sphere.sky_settings.cloud_settings.thickness;
        let previous_iterations = self.cellestial_sphere.sky_settings.cloud_settings.iterations;
        ui.label("Cloudiness")
            .on_hover_text("These settings dictate what the maximum increase in magnitude (decrease in brightness) should be due to clouds and how the clouds should look like");

        ui.horizontal(|ui| {
            ui.add(egui::Checkbox::without_text(&mut self.cellestial_sphere.sky_settings.cloud_settings.recalculate_on_change));
            ui.label("Recalculate on change of settings").on_hover_text("Should the clouds be recalculated when settings change?");
        });
        ui.horizontal(|ui| {
            ui.add(egui::Checkbox::without_text(&mut self.cellestial_sphere.sky_settings.cloud_settings.enabled));
            ui.label("Enabled").on_hover_text("Should there be any clouds?");
        });
        ui.horizontal(|ui| {
            ui.add(egui::DragValue::new(&mut self.cellestial_sphere.sky_settings.cloud_settings.coverage).speed(0.02));
            ui.label("Coverage").on_hover_text("How much of the sky (a fraction from 0 to 1) should be covered in clouds?");
        });
        ui.horizontal(|ui| {
            ui.add(egui::DragValue::new(&mut self.cellestial_sphere.sky_settings.cloud_settings.thickness).speed(0.1))
                .on_hover_text("How thick should the clouds be? More specifically, what should the maximum increase in magnitude due to clouds be?");
            ui.label("Thickness");
        });
        ui.horizontal(|ui| {
            ui.add(egui::DragValue::new(&mut self.cellestial_sphere.sky_settings.cloud_settings.iterations).speed(0.1))
                .on_hover_text("How detailed should the clouds be? Higher values lead to more structured clouds (1 corresponds to essentially blobs on the sky, 8 and higher actually look like clouds) but at the cost of longer computation times.");
            ui.label("Level of detail");
        });

        let recalculate = ui.button("Apply settings").clicked();

        self.cellestial_sphere.sky_settings.cloud_settings.clamp();

        if recalculate
            || (self.cellestial_sphere.sky_settings.cloud_settings.recalculate_on_change
                && (previous_enabled != self.cellestial_sphere.sky_settings.cloud_settings.enabled
                    || previous_coverage != self.cellestial_sphere.sky_settings.cloud_settings.coverage
                    || previous_thickness != self.cellestial_sphere.sky_settings.cloud_settings.thickness
                    || previous_iterations != self.cellestial_sphere.sky_settings.cloud_settings.iterations))
        {
            if self.cellestial_sphere.sky_settings.cloud_settings.enabled {
                crate::rendering::caspr::clouds::apply_dimming(&mut self.cellestial_sphere.stars, &self.cellestial_sphere.sky_settings.cloud_settings);
            } else {
                crate::rendering::caspr::clouds::disable(&mut self.cellestial_sphere.stars);
            }
            let keys = self.cellestial_sphere.stars.keys().cloned().collect::<Vec<String>>();
            for star_set_name in keys {
                self.cellestial_sphere.init_single_renderer_group(RendererCategory::Stars, &star_set_name);
            }
        }
    }

    pub fn render_sky_settings_stars_subwindow(&mut self, ui: &mut egui::Ui) {
        let override_rule_changed = ui.checkbox(&mut self.graphics_settings.use_overriden_star_colour, "Override the default star colour").changed();
        self.theme.game_visuals.use_overriden_star_colour = self.graphics_settings.use_overriden_star_colour;
        let override_colour_changed = ui
            .horizontal(|ui| {
                let changed = ui.color_edit_button_srgba(&mut self.theme.game_visuals.override_star_colour).changed();
                ui.label("Override star colour");
                changed
            })
            .inner;

        if override_rule_changed || (override_colour_changed && self.graphics_settings.use_overriden_star_colour) {
            let colour = if self.graphics_settings.use_overriden_star_colour {
                Some(self.theme.game_visuals.override_star_colour)
            } else {
                None
            };
            for star_set in self.cellestial_sphere.stars.values_mut() {
                for star in star_set {
                    star.override_colour = colour;
                }
            }
            let keys = self.cellestial_sphere.stars.keys().cloned().collect::<Vec<String>>();
            for star_set_name in keys {
                self.cellestial_sphere.init_single_renderer_group(RendererCategory::Stars, &star_set_name);
            }
        }

        let mut reinit_stars = false;
        let prev_mag_to_rad_fn_id = self.cellestial_sphere.sky_settings.mag_to_radius_id;
        ui.horizontal(|ui| {
            ui.label("Magnitude to radius function: ");
            egui::ComboBox::from_id_salt("Magnitude to radius function: ")
                .selected_text(self.cellestial_sphere.sky_settings.mag_to_radius_settings[self.cellestial_sphere.sky_settings.mag_to_radius_id].name())
                .show_ui(ui, |ui: &mut egui::Ui| {
                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                    for i in 0..self.cellestial_sphere.sky_settings.mag_to_radius_settings.len() {
                        ui.selectable_value(
                            &mut self.cellestial_sphere.sky_settings.mag_to_radius_id,
                            i,
                            self.cellestial_sphere.sky_settings.mag_to_radius_settings[i].name(),
                        );
                    }
                });
        });
        if prev_mag_to_rad_fn_id != self.cellestial_sphere.sky_settings.mag_to_radius_id {
            let place = CellestialSphere::mag_settings_to_light_pollution_place(
                self.cellestial_sphere.sky_settings.mag_to_radius_settings[self.cellestial_sphere.sky_settings.mag_to_radius_id],
                &self.cellestial_sphere.light_pollution_place_to_mag,
            );
            self.cellestial_sphere.light_pollution_place = place;
            reinit_stars = true;
        }

        match &mut self.cellestial_sphere.sky_settings.mag_to_radius_settings[self.cellestial_sphere.sky_settings.mag_to_radius_id] {
            stars::MagnitudeToRadius::Linear { mag_scale, mag_offset } => {
                let prev_mag_offset = *mag_offset;
                let prev_mag_scale = *mag_scale;
                ui.horizontal_wrapped(|ui| ui.label("The following two values affect the size of the stars via the following formula: radius = mag_scale * (mag_offset - magnitude)"));
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(mag_offset).speed(0.03));
                    ui.label("Magnitude offset (mag_offset)");
                });
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(mag_scale).speed(0.01));
                    ui.label("Magnitude scale (mag_scale)");
                });
                if prev_mag_offset != *mag_offset || prev_mag_scale != *mag_scale {
                    self.cellestial_sphere.light_pollution_place = CellestialSphere::mag_settings_to_light_pollution_place(
                        self.cellestial_sphere.sky_settings.mag_to_radius_settings[self.cellestial_sphere.sky_settings.mag_to_radius_id],
                        &self.cellestial_sphere.light_pollution_place_to_mag,
                    );
                    reinit_stars = true;
                }
            }
            stars::MagnitudeToRadius::Exponential { r_0, n, o } => {
                let prev_r0 = *r_0;
                let prev_n = *n;
                let prev_o = *o;
                ui.horizontal_wrapped(|ui| ui.label("The following three values affect the size of the stars via the following formula: radius = r_0 * ln(180°*n/fov) * 10^(-o*magnitude)"));
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(r_0).speed(0.03));
                    ui.label("r_0 (a size multiplier)");
                });
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(n).speed(0.01));
                    ui.label("n (how much does the size change (proportionally) when changing the FOV; higher values of n cause smaller changes)");
                });
                if *n < 2.0 {
                    *n = 2.0;
                }
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(o).speed(0.001));
                    ui.label("o (how much does the size change (proportionally) when changing the magnitude");
                });
                if prev_r0 != *r_0 || prev_n != *n || prev_o != *o {
                    self.cellestial_sphere.light_pollution_place = CellestialSphere::mag_settings_to_light_pollution_place(
                        self.cellestial_sphere.sky_settings.mag_to_radius_settings[self.cellestial_sphere.sky_settings.mag_to_radius_id],
                        &self.cellestial_sphere.light_pollution_place_to_mag,
                    );
                    reinit_stars = true;
                }
            }
        }

        let mut newly_active_star_groups = Vec::new();
        let mut newly_inactive_star_groups = Vec::new();
        for (name, active) in &mut self.cellestial_sphere.sky_settings.stars_categories_active {
            let active_before = *active;
            ui.checkbox(active, format!("Render stars from the {name} file"));
            if !active_before && *active {
                newly_active_star_groups.push(name.to_owned());
            } else if active_before && !*active {
                newly_inactive_star_groups.push(name.to_owned());
            }
        }

        for name in &newly_active_star_groups {
            self.cellestial_sphere.init_single_renderer_group(RendererCategory::Stars, name);
        }
        for name in &newly_inactive_star_groups {
            self.cellestial_sphere.deinit_single_renderer_group(RendererCategory::Stars, name);
        }
        if reinit_stars {
            self.cellestial_sphere.reinit_renderer_category(RendererCategory::Stars);
        }
    }

    pub fn render_sky_settings_deepsky_subwindow(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.add(egui::DragValue::new(&mut self.cellestial_sphere.sky_settings.deepsky_render_mag_decrease).speed(0.1));
            ui.label("Magnitude decrease")
                .on_hover_text("By how much should the magnitude of the deepsky objects be decreased for rendering - this way the objects can be made to be seen even without zooming in");
        });

        ui.checkbox(&mut self.cellestial_sphere.sky_settings.render_labels, "Render labels");

        let mut deepsky_groups_to_init = HashSet::new();
        let mut deepsky_groups_to_deinit = HashSet::new();
        for (name, deepskies_set) in &mut self.cellestial_sphere.deepskies {
            ui.heading(name);
            if ui.checkbox(&mut deepskies_set.active, format!("Render deepsky objects from the {name} file")).changed() {
                if deepskies_set.active {
                    deepsky_groups_to_init.insert(name.to_owned());
                } else {
                    deepsky_groups_to_deinit.insert(name.to_owned());
                }
                self.cellestial_sphere.sky_settings.deepskies_categories_active.insert(name.to_owned(), deepskies_set.active);
            }
            ui.horizontal(|ui| {
                ui.label("Marker colour: ");
                if ui.color_edit_button_srgba(&mut deepskies_set.colour).changed() {
                    deepsky_groups_to_init.insert(name.to_owned());
                }
            });
            self.theme.game_visuals.deepskies_colours.insert(name.clone(), deepskies_set.colour);
        }
        for name in &deepsky_groups_to_init {
            self.cellestial_sphere.init_single_renderer_group(RendererCategory::Deepskies, name);
        }
        for name in &deepsky_groups_to_deinit {
            self.cellestial_sphere.deinit_single_renderer_group(RendererCategory::Deepskies, name);
        }
    }

    pub fn render_sky_settings_lines_subwindow(&mut self, ui: &mut egui::Ui) {
        let mut line_groups_to_init = HashSet::new();
        let mut line_groups_to_deinit = HashSet::new();
        for (name, lines_set) in &mut self.cellestial_sphere.lines {
            ui.heading(name);
            if ui.checkbox(&mut lines_set.active, format!("Render lines from the {name} file")).changed() {
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
            self.cellestial_sphere.init_single_renderer_group(RendererCategory::Lines, name);
        }
        for name in &line_groups_to_deinit {
            self.cellestial_sphere.deinit_single_renderer_group(RendererCategory::Lines, name);
        }
    }

    pub fn render_sky_settings_markers_subwindow(&mut self, ui: &mut egui::Ui) {
        let mut game_markers_changed = false;
        ui.heading("Game markers");
        ui.horizontal(|ui| {
            ui.label("Guess marker colour: ");
            game_markers_changed |= ui.color_edit_button_srgba(&mut self.theme.game_visuals.game_markers_colours.exact).changed();
        });
        ui.horizontal(|ui| {
            ui.label("Tolerance marker colour: ");
            game_markers_changed |= ui.color_edit_button_srgba(&mut self.theme.game_visuals.game_markers_colours.tolerance).changed();
        });
        ui.horizontal(|ui| {
            ui.label("Task marker colour: ");
            game_markers_changed |= ui.color_edit_button_srgba(&mut self.theme.game_visuals.game_markers_colours.task).changed();
        });
        ui.horizontal(|ui| {
            ui.label("Corrent answer marker colour: ");
            game_markers_changed |= ui.color_edit_button_srgba(&mut self.theme.game_visuals.game_markers_colours.correct_answer).changed();
        });
        if game_markers_changed {
            for marker in self.cellestial_sphere.game_markers.markers.iter_mut() {
                marker.colour = GameMarker::get_colour(marker.marker_type, &self.theme.game_visuals.game_markers_colours);
            }
            self.cellestial_sphere.init_single_renderer_group(RendererCategory::Markers, "game");
        }
        ui.separator();
        let mut marker_groups_to_init = HashSet::new();
        let mut marker_groups_to_deinit = HashSet::new();
        for (name, markers_set) in &mut self.cellestial_sphere.markers {
            ui.heading(name);
            if ui.checkbox(&mut markers_set.active, format!("Render markers from the {name} file")).changed() {
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
            self.cellestial_sphere.init_single_renderer_group(RendererCategory::Markers, name);
        }
        for name in &marker_groups_to_deinit {
            self.cellestial_sphere.deinit_single_renderer_group(RendererCategory::Markers, name);
        }
    }
}
