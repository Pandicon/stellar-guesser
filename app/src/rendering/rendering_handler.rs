use crate::{rendering::initial_setup, Application};
use eframe::egui;

impl Application {
    pub fn render(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) -> bool {
        let viewport_rect = ctx.input(|i| i.screen_rect());
        initial_setup::render_initial_setup(self, ctx, viewport_rect);

        let mut window_rectangles = Vec::new();
        if let Some(response) = self.render_application_info_window(ctx) {
            window_rectangles.push([
                [response.response.rect.right(), response.response.rect.top()],
                [response.response.rect.left(), response.response.rect.bottom()],
            ]);
        }
        if let Some(response) = self.render_credits_window(ctx) {
            window_rectangles.push([
                [response.response.rect.right(), response.response.rect.top()],
                [response.response.rect.left(), response.response.rect.bottom()],
            ]);
        }
        if let Some(response) = self.render_feedback_and_support_window(ctx) {
            window_rectangles.push([
                [response.response.rect.right(), response.response.rect.top()],
                [response.response.rect.left(), response.response.rect.bottom()],
            ]);
        }
        if let Some(response) = self.render_settings_window(ctx) {
            window_rectangles.push([
                [response.response.rect.right(), response.response.rect.top()],
                [response.response.rect.left(), response.response.rect.bottom()],
            ]);
        }
        if let Some(response) = self.render_statistics_window(ctx) {
            window_rectangles.push([
                [response.response.rect.right(), response.response.rect.top()],
                [response.response.rect.left(), response.response.rect.bottom()],
            ]);
        }
        if let Some(response) = self.render_question_window(ctx) {
            window_rectangles.push([
                [response.response.rect.right(), response.response.rect.top()],
                [response.response.rect.left(), response.response.rect.bottom()],
            ]);
        }
        if let Some(response) = self.render_testing_window(ctx) {
            window_rectangles.push([
                [response.response.rect.right(), response.response.rect.top()],
                [response.response.rect.left(), response.response.rect.bottom()],
            ]);
        }
        let mut panel_frame = egui::Frame::central_panel(&ctx.style());
        panel_frame.inner_margin = egui::Margin::same(0);
        let central_panel_response = egui::CentralPanel::default()
            .frame(panel_frame)
            .show(ctx, |ui| {
                let rect = ui.available_rect_before_wrap();
                if rect != self.cellestial_sphere.camera.viewport_rect {
                    log::debug!("Viewport rect changed: {:?} -> {:?}", self.cellestial_sphere.camera.viewport_rect, rect);
                    self.cellestial_sphere.camera.viewport_rect = rect;
                    self.cellestial_sphere.camera.changed_viewport_rect = true;
                }
                self.cellestial_sphere.prepare_render(&self.sky);
                let painter = ui.painter();
                self.cellestial_sphere.render_sky(painter, frame);
                self.cellestial_sphere.after_render();
            })
            .response
            .interact(egui::Sense::click_and_drag());
        let top_panel_hovered = self.render_top_panel(ctx);
        log::debug!("Top panel hovered: {top_panel_hovered}");
        // The central panel is hovered and the top panel is not
        central_panel_response.contains_pointer() && !top_panel_hovered
    }
}
