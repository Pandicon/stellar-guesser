use crate::{enums::RendererCategory, structs::state::windows::settings::GameSettingsSubWindow, Application};
use eframe::egui;

pub mod general;
pub mod questions;

impl Application {
    pub fn render_game_settings_window(&mut self, ui: &mut egui::Ui) {
        let mut tolerance_changed = false;
        ui.horizontal(|ui| {
            ui.selectable_value(
                &mut self.state.windows.settings.game_settings.subwindow,
                GameSettingsSubWindow::General,
                GameSettingsSubWindow::General.as_ref(),
            );
            ui.selectable_value(
                &mut self.state.windows.settings.game_settings.subwindow,
                GameSettingsSubWindow::Questions,
                GameSettingsSubWindow::Questions.as_ref(),
            );
            ui.selectable_value(
                &mut self.state.windows.settings.game_settings.subwindow,
                GameSettingsSubWindow::Constellations,
                GameSettingsSubWindow::Constellations.as_ref(),
            );
        });
        ui.separator();
        match self.state.windows.settings.game_settings.subwindow {
            GameSettingsSubWindow::General => {
                egui::ScrollArea::vertical().auto_shrink([false, true]).show(ui, |ui| self.render_game_settings_general_subwindow(ui));
            }
            GameSettingsSubWindow::Questions => {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, true])
                    .show(ui, |ui| self.render_game_settings_questions_subwindow(ui, &mut tolerance_changed));
            }
            GameSettingsSubWindow::Constellations => {
                let mut abbrev_to_name = std::collections::HashMap::new();
                for constellation in self.cellestial_sphere.constellations.values() {
                    abbrev_to_name.insert(constellation.abbreviation.clone(), constellation.possible_names[1].clone());
                }
                sg_game_constellations::ui::render_constellations_settings_subwindow(
                    ui,
                    &mut self.state.windows.settings.sky_settings.groups_subwindow_state,
                    &mut self.game_handler.constellation_groups_settings,
                    abbrev_to_name,
                );
                // self.render_game_settings_constellations_subwindow(ui)
            }
        };
        if tolerance_changed && self.game_handler.show_tolerance_marker() {
            let markers = self.game_handler.generate_player_markers(&self.game_handler.guess_marker_positions, &self.theme);
            self.cellestial_sphere.game_markers.markers = markers;
            self.cellestial_sphere.init_single_renderer_group(RendererCategory::Markers, "game");
        }
    }
}
