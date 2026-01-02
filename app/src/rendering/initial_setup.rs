use eframe::egui;

use crate::Application;

#[derive(serde::Deserialize, serde::Serialize, Default, PartialEq, Eq)]
pub enum InitialSetupStage {
    Finished,
    #[cfg(not(target_arch = "wasm32"))]
    Keyboard,
    Community,
    Credits,
    #[default]
    Introduction,
}

#[cfg(not(target_arch = "wasm32"))]
impl InitialSetupStage {
    pub fn next(&self) -> Option<Self> {
        match *self {
            Self::Introduction => Some(Self::Keyboard),
            Self::Keyboard => Some(Self::Community),
            Self::Community => Some(Self::Credits),
            Self::Credits => Some(Self::Finished),
            Self::Finished => None,
        }
    }

    pub fn previous(&self) -> Option<Self> {
        match *self {
            Self::Introduction => None,
            Self::Keyboard => Some(Self::Introduction),
            Self::Community => Some(Self::Keyboard),
            Self::Credits => Some(Self::Community),
            Self::Finished => Some(Self::Credits),
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl InitialSetupStage {
    pub fn next(&self) -> Option<Self> {
        match *self {
            Self::Introduction => Some(Self::Community),
            Self::Community => Some(Self::Credits),
            Self::Credits => Some(Self::Finished),
            Self::Finished => None,
        }
    }

    pub fn previous(&self) -> Option<Self> {
        match *self {
            Self::Introduction => None,
            Self::Community => Some(Self::Introduction),
            Self::Credits => Some(Self::Community),
            Self::Finished => Some(Self::Credits),
        }
    }
}

impl InitialSetupStage {
    pub fn will_next_finish(&self) -> bool {
        self.next() == Some(Self::Finished)
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
                        if let Some(next_stage) = app.initial_setup_stage.next() {
                            let button_text = if app.initial_setup_stage.will_next_finish() {"Finish"} else {"Next"};
                            if ui.button(button_text).clicked() {
                                app.initial_setup_stage = next_stage;
                            }
                        }
                    },
                );
            });
        }
        #[cfg(not(target_arch = "wasm32"))]
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
                        if let Some(next_stage) = app.initial_setup_stage.next() {
                            let button_text = if app.initial_setup_stage.will_next_finish() { "Finish" } else { "Next" };
                            if ui.button(button_text).clicked() {
                                app.initial_setup_stage = next_stage;
                            }
                        }
                        if let Some(previous_stage) = app.initial_setup_stage.previous() {
                            if ui.button("Back").clicked() {
                                app.initial_setup_stage = previous_stage;
                            }
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
                        if let Some(next_stage) = app.initial_setup_stage.next() {
                            let button_text = if app.initial_setup_stage.will_next_finish() { "Finish" } else { "Next" };
                            if ui.button(button_text).clicked() {
                                app.initial_setup_stage = next_stage;
                            }
                        }
                        if let Some(previous_stage) = app.initial_setup_stage.previous() {
                            if ui.button("Back").clicked() {
                                app.initial_setup_stage = previous_stage;
                            }
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
                        if let Some(next_stage) = app.initial_setup_stage.next() {
                            let button_text = if app.initial_setup_stage.will_next_finish() { "Finish" } else { "Next" };
                            if ui.button(button_text).clicked() {
                                app.initial_setup_stage = next_stage;
                            }
                        }
                        if let Some(previous_stage) = app.initial_setup_stage.previous() {
                            if ui.button("Back").clicked() {
                                app.initial_setup_stage = previous_stage;
                            }
                        }
                    },
                );
            });
        }
    }
}
