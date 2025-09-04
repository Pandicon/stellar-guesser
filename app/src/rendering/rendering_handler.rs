use crate::Application;
use eframe::egui;

impl Application {
    pub fn render(&mut self, ctx: &egui::Context) -> bool {
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
        let viewport_rect = ctx.input(|i| i.screen_rect());
        if viewport_rect != self.cellestial_sphere.viewport_rect {
            log::debug!("Viewport rect changed: {:?} -> {:?}", self.cellestial_sphere.viewport_rect, viewport_rect);
            self.cellestial_sphere.viewport_rect = viewport_rect;
            self.cellestial_sphere.init_renderers();
        }
        let central_panel_response = egui::CentralPanel::default()
            .show(ctx, |ui| {
                self.cellestial_sphere.viewport_rect = viewport_rect;

                let painter = ui.painter();
                self.cellestial_sphere.render_sky(painter);
            })
            .response
            .interact(egui::Sense::click_and_drag());
        let top_panel_hovered = self.render_top_panel(ctx);
        log::debug!("Top panel hovered: {top_panel_hovered}");
        // The central panel is hovered and the top panel is not
        central_panel_response.contains_pointer() && !top_panel_hovered
    }
}
