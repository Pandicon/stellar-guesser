use crate::{enums::LightPollution, Application};
use eframe::egui;

impl Application {
    pub fn render_top_panel(&mut self, ctx: &egui::Context) -> bool {
        egui::TopBottomPanel::top("top_panel")
            .show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.label(format!("FOV: {:.3}°", self.cellestial_sphere.fov));
                        if !self.screen_width.very_narrow() {
                            render_left_controls(self, ui);
                        }
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if self.screen_width.narrow() {
                            ui.menu_button("Settings & Controls", |ui| {
                                if self.screen_width.very_narrow() {
                                    render_left_controls(self, ui);
                                }
                                render_right_controls(self, ui);
                            });
                        } else {
                            render_right_controls(self, ui);
                        }
                    });
                });
            })
            .response
            //.interact(egui::Sense::click_and_drag()) // Inner widgets become unclickable...
            .hovered()
    }
}

fn render_right_controls(app: &mut crate::application::Application, ui: &mut egui::Ui) {
    let app_info_btn = ui
        .add(egui::Button::new(egui::RichText::new("App info").text_style(egui::TextStyle::Body)))
        .on_hover_text("Show information about the application");
    if app_info_btn.clicked() {
        app.state.windows.app_info.opened = true;
    }
    let credits_btn = ui
        .add(egui::Button::new(egui::RichText::new("Credits").text_style(egui::TextStyle::Body)))
        .on_hover_text("Show the credits for the resources used by this application");
    if credits_btn.clicked() {
        app.state.windows.credits.opened = true;
    }
    let stats_btn: egui::Response = ui
        .add(egui::Button::new(egui::RichText::new("Statistics").text_style(egui::TextStyle::Body)))
        .on_hover_text("Show your statistics");
    if stats_btn.clicked() {
        app.state.windows.stats.opened = true;
    }
    let settings_btn = ui
        .add(egui::Button::new(egui::RichText::new("Settings").text_style(egui::TextStyle::Body)))
        .on_hover_text("Show the settings");
    if settings_btn.clicked() {
        app.state.windows.settings.opened = true;
    }
    let game_question_btn = ui
        .add(egui::Button::new(egui::RichText::new("Question").text_style(egui::TextStyle::Body)))
        .on_hover_text("Show the question");
    if game_question_btn.clicked() {
        app.state.windows.game_question.opened = true;
    }

    if app.testing_mode {
        let testing_btn = ui
            .add(egui::Button::new(egui::RichText::new("Testing").text_style(egui::TextStyle::Body)))
            .on_hover_text("Show visual tests");
        if testing_btn.clicked() {
            app.state.windows.testing.opened = true;
        }
    }
}

fn render_left_controls(app: &mut crate::application::Application, ui: &mut egui::Ui) {
    ui.label(app.frames_handler.fps_display_holder.clone());
    ui.label(app.frames_handler.average_fps_display_holder.clone()).on_hover_text(format!(
        "The average FPS over the last {} frame{}",
        app.frames_handler.frames_analysed,
        if app.frames_handler.frames_analysed != 1 { "s" } else { "" }
    ));
    let prev_light_pollution: LightPollution = app.cellestial_sphere.light_pollution_place;
    ui.horizontal(|ui| {
        ui.label("Light pollution level: ")
            .on_hover_text("These settings are made to reflect how the sky looks in different locations for a person with an average eyesight.");
        egui::ComboBox::from_id_salt("Light pollution level: ")
            .selected_text(format!("{}", app.cellestial_sphere.light_pollution_place))
            .show_ui(ui, |ui| {
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                for val in LightPollution::variants() {
                    ui.selectable_value(&mut app.cellestial_sphere.light_pollution_place, val, format!("{}", val))
                        .on_hover_text(LightPollution::explanation(&val));
                }
            });
    });
    if prev_light_pollution != app.cellestial_sphere.light_pollution_place {
        let settings = app.cellestial_sphere.light_pollution_place_to_mag_settings(&app.cellestial_sphere.light_pollution_place);
        app.cellestial_sphere.sky_settings.mag_to_radius_settings[app.cellestial_sphere.sky_settings.mag_to_radius_id] = settings;
        app.cellestial_sphere.reinit_renderer_category(crate::enums::RendererCategory::Stars);
    }
}
