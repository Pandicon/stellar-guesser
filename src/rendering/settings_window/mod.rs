use crate::{structs::state::windows::settings::SettingsSubWindow, Application};

pub mod game_settings_window;
pub mod sky_settings_window;

impl Application {
    pub fn render_settings_window(&mut self, ctx: &egui::Context) -> Option<egui::InnerResponse<Option<()>>> {
        let mut opened = self.state.windows.settings.opened;
        let response = egui::Window::new("Settings").open(&mut opened).show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.state.windows.settings.subwindow, SettingsSubWindow::Game, SettingsSubWindow::Game.as_ref());
                ui.selectable_value(&mut self.state.windows.settings.subwindow, SettingsSubWindow::Sky, SettingsSubWindow::Sky.as_ref());
            });
            ui.separator();
            match self.state.windows.settings.subwindow {
                SettingsSubWindow::Game => self.render_game_settings_window(ui),
                SettingsSubWindow::Sky => self.render_sky_settings_window(ctx, ui),
            }
        });
        self.state.windows.settings.opened = opened;
        response
    }
}
