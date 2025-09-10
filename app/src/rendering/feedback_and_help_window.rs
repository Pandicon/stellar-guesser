use eframe::egui;

use crate::Application;

impl Application {
    pub fn render_feedback_and_support_window(&mut self, ctx: &egui::Context) -> Option<egui::InnerResponse<Option<()>>> {
        egui::Window::new("Provide feedback and get support")
            .open(&mut self.state.windows.feedback_and_help.opened)
            .show(ctx, |ui| {
                render_feedback_and_support_window_inner(ui);
            })
    }
}

pub fn render_feedback_and_support_window_inner(ui: &mut egui::Ui) {
    ui.label("We would love to get some feedback from you! Whether you have encountered an issue, want to suggest a new feature, need help with the app or learning the sky in general, or just want to join our community and have a chat, you can join us at the following places:");
    ui.heading("Discord server");
    ui.hyperlink(&crate::CONFIG.discord_server_invite);
}
