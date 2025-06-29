use crate::Application;
use eframe::egui;

impl Application {
    pub fn render_statistics_window(&mut self, ctx: &egui::Context) -> Option<egui::InnerResponse<Option<()>>> {
        let seconds_spent = self.state.time_spent_start + (self.frame_timestamp - self.state.start_timestamp);
        let mut time_spent_changing = seconds_spent;
        let seconds = time_spent_changing % 60;
        time_spent_changing -= seconds;
        time_spent_changing /= 60;
        let minutes = time_spent_changing % 60;
        time_spent_changing -= minutes;
        time_spent_changing /= 60;
        let hours = time_spent_changing;
        egui::Window::new("Statistics").open(&mut self.state.windows.stats.opened).show(ctx, |ui| {
            ui.label(format!("Time spent in the application: {hours}h {minutes}min {seconds}s"));
        })
    }
}
