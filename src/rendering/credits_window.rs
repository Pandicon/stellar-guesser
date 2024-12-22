use eframe::egui;

use crate::Application;

impl Application {
    pub fn render_credits_window(&mut self, ctx: &egui::Context) -> Option<egui::InnerResponse<Option<()>>> {
        sg_credits::ui::render_credits_window(&mut self.state.windows.credits.opened, ctx)
    }
}
