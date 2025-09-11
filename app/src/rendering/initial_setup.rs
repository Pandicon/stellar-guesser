use eframe::egui;

use crate::Application;

#[derive(serde::Deserialize, serde::Serialize)]
pub enum InitialSetupStage {
    Finished,
    Keyboard,
    Community,
    Credits,
    Introduction,
}

impl Default for InitialSetupStage {
    fn default() -> Self {
        Self::Introduction
    }
}

pub fn render_initial_setup(app: &mut Application, ctx: &egui::Context, available_rect: egui::Rect) {
    let available_width = (available_rect.max.x - available_rect.min.x).abs();
    let available_height = (available_rect.max.y - available_rect.min.y).abs();
    let (modal_width, modal_height) = match app.screen_width {
        crate::enums::ScreenWidth::Normal => (available_width / 3.0, available_height * 0.5),
        crate::enums::ScreenWidth::Narrow => (available_width * 0.6, available_height * 0.5),
        crate::enums::ScreenWidth::VeryNarrow => (available_width * 0.9, available_height * 0.5),
    };
    let top_offset = available_height * 0.0;
    match app.initial_setup_stage {
        InitialSetupStage::Finished => {}
        InitialSetupStage::Introduction => {
            let modal = egui::Modal::new(egui::Id::new("Onboarding"));
            let modal_area = modal.area.anchor(egui::Align2::CENTER_TOP, [0.0, top_offset]).order(egui::Order::Middle);
            modal.area(modal_area).show(ctx, |ui| {
                ui.set_width(modal_width);
                ui.set_max_height(modal_height);
                ui.heading("Onboarding");
                egui::ScrollArea::vertical().auto_shrink([false, true]).show(ui, |ui| {
                    ui.label("The following screens will guide you through some essential setup and information you may find useful. Everything that gets configured now can later be changed in the settings window, where you can also find the rest of the settings that are not shown here.")
                });
                egui::Sides::new().show(
                    ui,
                    |_ui| {},
                    |ui| {
                        if ui.button("Next").clicked() {
                            app.initial_setup_stage = InitialSetupStage::Keyboard;
                        }
                    },
                );
            });
        }
        InitialSetupStage::Keyboard => {
            let modal = egui::Modal::new(egui::Id::new("Onboarding - Keyboard setup"));
            let modal_area = modal.area.anchor(egui::Align2::CENTER_TOP, [0.0, top_offset]).order(egui::Order::Middle);
            modal.area(modal_area).show(ctx, |ui| {
                ui.set_width(modal_width);
                ui.set_max_height(modal_height);
                ui.heading("Onboarding - Keyboard setup");
                egui::ScrollArea::vertical().auto_shrink([false, true]).show(ui, |ui| {
                    app.render_application_settings_input_subwindow(ui);
                });
                egui::Sides::new().show(
                    ui,
                    |_ui| {},
                    |ui| {
                        if ui.button("Next").clicked() {
                            app.initial_setup_stage = InitialSetupStage::Community;
                        }
                        if ui.button("Back").clicked() {
                            app.initial_setup_stage = InitialSetupStage::Introduction;
                        }
                    },
                );
            });
        }
        InitialSetupStage::Community => {
            let modal = egui::Modal::new(egui::Id::new("Onboarding - Community"));
            let modal_area = modal.area.anchor(egui::Align2::CENTER_TOP, [0.0, top_offset]).order(egui::Order::Middle);
            modal.area(modal_area).show(ctx, |ui| {
                ui.set_width(modal_width);
                ui.set_max_height(modal_height);
                ui.heading("Onboarding - Community");
                egui::ScrollArea::vertical().auto_shrink([false, true]).show(ui, |ui| {
                    crate::rendering::feedback_and_help_window::render_feedback_and_support_window_inner(ui);
                });
                egui::Sides::new().show(
                    ui,
                    |_ui| {},
                    |ui| {
                        if ui.button("Next").clicked() {
                            app.initial_setup_stage = InitialSetupStage::Credits;
                        }
                        if ui.button("Back").clicked() {
                            app.initial_setup_stage = InitialSetupStage::Keyboard;
                        }
                    },
                );
            });
        }
        InitialSetupStage::Credits => {
            let modal = egui::Modal::new(egui::Id::new("Onboarding - Credits"));
            let modal_area = modal.area.anchor(egui::Align2::CENTER_TOP, [0.0, top_offset]).order(egui::Order::Middle);
            modal.area(modal_area).show(ctx, |ui| {
                ui.set_width(modal_width);
                ui.set_max_height(modal_height);
                ui.heading("Onboarding - Credits");
                sg_credits::ui::render_credits_inner(ui);
                egui::Sides::new().show(
                    ui,
                    |_ui| {},
                    |ui| {
                        if ui.button("Finish").clicked() {
                            app.initial_setup_stage = InitialSetupStage::Finished;
                        }
                        if ui.button("Back").clicked() {
                            app.initial_setup_stage = InitialSetupStage::Community;
                        }
                    },
                );
            });
        }
    }
}
