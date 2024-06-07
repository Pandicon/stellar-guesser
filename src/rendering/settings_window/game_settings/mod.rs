use crate::{structs::state::windows::settings::GameSettingsSubWindow, Application};

pub mod constellations;
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
        egui::ScrollArea::vertical()
            .auto_shrink([false, true])
            .show(ui, |ui| match self.state.windows.settings.game_settings.subwindow {
                GameSettingsSubWindow::General => self.render_game_settings_general_subwindow(ui),
                GameSettingsSubWindow::Questions => self.render_game_settings_questions_subwindow(ui, &mut tolerance_changed),
                GameSettingsSubWindow::Constellations => self.render_game_settings_constellations_subwindow(ui),
            });
        if tolerance_changed && self.game_handler.show_tolerance_marker() {
            let markers = self.game_handler.generate_player_markers(&self.game_handler.guess_marker_positions);
            self.cellestial_sphere.markers.insert("game".to_string(), markers);
            self.cellestial_sphere.init_single_renderer("markers", "game");
        }
    }
}
