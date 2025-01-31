use crate::{structs::state::windows::settings::GameSettingsQuestionsSubWindow, structs::state::windows::settings::GameSettingsType, Application};
use angle::Angle;
use eframe::egui;

impl Application {
    pub fn render_game_settings_questions_subwindow(&mut self, ui: &mut egui::Ui, tolerance_changed: &mut bool) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.state.windows.settings.game_settings.settings_type, GameSettingsType::Basic, GameSettingsType::Basic.as_ref());
            ui.selectable_value(
                &mut self.state.windows.settings.game_settings.settings_type,
                GameSettingsType::Advanced,
                GameSettingsType::Advanced.as_ref(),
            );
        });
        ui.separator();

        match self.state.windows.settings.game_settings.settings_type {
            GameSettingsType::Basic => {
                ui.horizontal(|ui| {
                    // If adding new question types, make sure that the picker gets collapsed into a combo box on an appropriately wide/narrow screens
                    if self.screen_width.narrow() {
                        ui.label("Question type: ");
                        egui::ComboBox::from_id_salt("Question type: ")
                            .selected_text(format!("{}", self.state.windows.settings.game_settings.questions_subwindow.subwindow))
                            .show_ui(ui, |ui: &mut egui::Ui| {
                                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                                self.render_question_type_picker(ui);
                            });
                    } else {
                        self.render_question_type_picker(ui);
                    }
                });
                ui.separator();
                match self.state.windows.settings.game_settings.questions_subwindow.subwindow {
                    GameSettingsQuestionsSubWindow::FindThisObject => self.render_game_settings_find_this_object_subwindow(ui, tolerance_changed),
                    GameSettingsQuestionsSubWindow::WhatIsThisObject => self.render_game_settings_what_is_this_object_subwindow(ui),
                    GameSettingsQuestionsSubWindow::WhichConstellationIsThisPointIn => self.render_game_settings_guess_the_constellation_subwindow(ui),
                    GameSettingsQuestionsSubWindow::GuessTheAngularDistance => self.render_game_settings_angular_distance_subwindow(ui),
                    GameSettingsQuestionsSubWindow::GuessTheCoordinates => self.render_game_settings_coordinates_subwindow(ui),
                    GameSettingsQuestionsSubWindow::GuessTheMagnitude => self.render_game_settings_magnitude_subwindow(ui),
                }

                self.state.windows.settings.game_settings.generated_query = self.generate_query_from_basic();

                ui.separator();
                ui.label("Generated query:");
                ui.label(egui::RichText::new(&self.state.windows.settings.game_settings.generated_query).code());
            }
            GameSettingsType::Advanced => {
                ui.label("Advanced settings will go here");
            }
        }
    }

    fn generate_query_from_basic(&self) -> String {
        let mut query = String::new();
        if self.game_handler.questions_settings.find_this_object.show {
            let mut args = vec![self.game_handler.questions_settings.find_this_object.correctness_threshold.to_deg().as_value().to_string()];
            if self.game_handler.questions_settings.find_this_object.rotate_to_correct_point {
                args.push(String::from("ROTATE"));
            }
            if self.game_handler.questions_settings.find_this_object.replay_incorrect {
                args.push(String::from("REPLAY"));
            }
            let mut settings_catalogues = Vec::new();
            if self.game_handler.questions_settings.find_this_object.show_messiers {
                settings_catalogues.push("CATALOGUE(MESSIER)");
            }
            if self.game_handler.questions_settings.find_this_object.show_caldwells {
                settings_catalogues.push("CATALOGUE(CALDWELL)");
            }
            if self.game_handler.questions_settings.find_this_object.show_ngcs {
                settings_catalogues.push("CATALOGUE(NGC)");
            }
            if self.game_handler.questions_settings.find_this_object.show_ics {
                settings_catalogues.push("CATALOGUE(IC)");
            }
            if self.game_handler.questions_settings.find_this_object.show_bayer {
                settings_catalogues.push("CATALOGUE(BAYER)");
            }
            if self.game_handler.questions_settings.find_this_object.show_starnames {
                settings_catalogues.push("AND(TYPE(STAR), CATALOGUE(PROPER_NAME))");
            }
            if !settings_catalogues.is_empty() {
                let settings = format!(
                    "OR(AND(TYPE(STAR), MAG_BELOW({}), OR({})), AND(NOT(TYPE(STAR)), OR({})))",
                    self.game_handler.questions_settings.find_this_object.correctness_threshold.value(),
                    settings_catalogues.join(", "),
                    settings_catalogues.join(", ")
                );
                query = format!("FIND_THIS_OBJECT({}): {}", args.join(", "), settings);
            };
        }

        if self.game_handler.questions_settings.what_constellation_is_this_point_in.show {
            query = format!(
                "{query}\nWHAT_CONSTELLATION_IS_THIS_POINT_IN({})",
                if self.game_handler.questions_settings.what_constellation_is_this_point_in.rotate_to_point {
                    "ROTATE"
                } else {
                    ""
                }
            );
        }
        if self.game_handler.questions_settings.angular_separation.show {
            query = format!(
                "{query}\nANGULAR_SEPARATION({})",
                if self.game_handler.questions_settings.angular_separation.rotate_to_midpoint {
                    "ROTATE"
                } else {
                    ""
                }
            );
        }
        if self.game_handler.questions_settings.guess_rad_dec.show {
            query = format!("{query}\nDEC({})", if self.game_handler.questions_settings.guess_rad_dec.rotate_to_point { "ROTATE" } else { "" });
            query = format!("{query}\nRA({})", if self.game_handler.questions_settings.guess_rad_dec.rotate_to_point { "ROTATE" } else { "" });
        }
        if self.game_handler.questions_settings.guess_the_magnitude.show {
            let mut args = Vec::new();
            if self.game_handler.questions_settings.guess_the_magnitude.rotate_to_point {
                args.push("ROTATE")
            };
            if self.game_handler.questions_settings.guess_the_magnitude.replay_incorrect {
                args.push("REPLAY");
            }
            query = format!(
                "{query}\nGUESS_THE_MAGNITUDE({}): AND(TYPE(STAR), MAG_BELOW({}))",
                args.join(", "),
                self.game_handler.questions_settings.guess_the_magnitude.magnitude_cutoff
            );
        }
        query
    }

    // If adding new question types, make sure that the picker gets collapsed into a combo box on an appropriately wide/narrow screens
    fn render_question_type_picker(&mut self, ui: &mut egui::Ui) {
        ui.selectable_value(
            &mut self.state.windows.settings.game_settings.questions_subwindow.subwindow,
            GameSettingsQuestionsSubWindow::FindThisObject,
            GameSettingsQuestionsSubWindow::FindThisObject.as_ref(),
        );
        ui.selectable_value(
            &mut self.state.windows.settings.game_settings.questions_subwindow.subwindow,
            GameSettingsQuestionsSubWindow::WhatIsThisObject,
            GameSettingsQuestionsSubWindow::WhatIsThisObject.as_ref(),
        );
        ui.selectable_value(
            &mut self.state.windows.settings.game_settings.questions_subwindow.subwindow,
            GameSettingsQuestionsSubWindow::WhichConstellationIsThisPointIn,
            GameSettingsQuestionsSubWindow::WhichConstellationIsThisPointIn.as_ref(),
        );
        ui.selectable_value(
            &mut self.state.windows.settings.game_settings.questions_subwindow.subwindow,
            GameSettingsQuestionsSubWindow::GuessTheAngularDistance,
            GameSettingsQuestionsSubWindow::GuessTheAngularDistance.as_ref(),
        );
        ui.selectable_value(
            &mut self.state.windows.settings.game_settings.questions_subwindow.subwindow,
            GameSettingsQuestionsSubWindow::GuessTheCoordinates,
            GameSettingsQuestionsSubWindow::GuessTheCoordinates.as_ref(),
        );
        ui.selectable_value(
            &mut self.state.windows.settings.game_settings.questions_subwindow.subwindow,
            GameSettingsQuestionsSubWindow::GuessTheMagnitude,
            GameSettingsQuestionsSubWindow::GuessTheMagnitude.as_ref(),
        );
    }

    fn render_game_settings_find_this_object_subwindow(&mut self, ui: &mut egui::Ui, tolerance_changed: &mut bool) {
        ui.checkbox(&mut self.game_handler.questions_settings.find_this_object.show, "Show the 'Find this object' questions");
        ui.checkbox(
            &mut self.game_handler.questions_settings.find_this_object.rotate_to_correct_point,
            "Rotate to the correct point after answering",
        )
        .on_hover_text("Whether or not to rotate the view so that the correct point is in the centre of the screen after answering");
        ui.checkbox(&mut self.game_handler.questions_settings.find_this_object.show_messiers, "Show Messier objects");
        ui.checkbox(&mut self.game_handler.questions_settings.find_this_object.show_caldwells, "Show Caldwell objects");
        ui.checkbox(&mut self.game_handler.questions_settings.find_this_object.show_ngcs, "Show NGC objects");
        ui.checkbox(&mut self.game_handler.questions_settings.find_this_object.show_ics, "Show IC objects");
        ui.checkbox(&mut self.game_handler.questions_settings.find_this_object.show_bayer, "Show Bayer designations");
        ui.checkbox(&mut self.game_handler.questions_settings.find_this_object.show_starnames, "Show star names");
        ui.add(egui::Slider::new(&mut self.game_handler.questions_settings.find_this_object.magnitude_cutoff, 0.0..=20.0).text("Star magnitude cutoff"));
        let mut correctness_threshold_inner = self.game_handler.questions_settings.find_this_object.correctness_threshold.value();
        let correctness_threshold_widget = ui.add(
            egui::Slider::new(&mut correctness_threshold_inner, 0.0..=180.0)
                .text("Correctness threshold (degrees)")
                .logarithmic(true),
        );
        self.game_handler.questions_settings.find_this_object.correctness_threshold = angle::Deg(correctness_threshold_inner);
        *tolerance_changed |= correctness_threshold_widget.changed();
        ui.checkbox(&mut self.game_handler.questions_settings.find_this_object.replay_incorrect, "Replay incorrectly answered questions");
    }

    fn render_game_settings_what_is_this_object_subwindow(&mut self, ui: &mut egui::Ui) {
        ui.checkbox(&mut self.game_handler.questions_settings.what_is_this_object.show, "Show the 'What is this object' questions");
        ui.checkbox(&mut self.game_handler.questions_settings.what_is_this_object.rotate_to_point, "Rotate to the point in question")
            .on_hover_text("Whether or not to rotate the view so that the point in question is in the centre of the screen");
        ui.checkbox(&mut self.game_handler.questions_settings.what_is_this_object.show_messiers, "Show Messier objects");
        ui.checkbox(&mut self.game_handler.questions_settings.what_is_this_object.show_caldwells, "Show Caldwell objects");
        ui.checkbox(&mut self.game_handler.questions_settings.what_is_this_object.show_ngcs, "Show NGC objects");
        ui.checkbox(&mut self.game_handler.questions_settings.what_is_this_object.show_ics, "Show IC objects");
        ui.checkbox(&mut self.game_handler.questions_settings.what_is_this_object.show_bayer, "Show Bayer designations");
        ui.checkbox(&mut self.game_handler.questions_settings.what_is_this_object.show_starnames, "Show star names");
        ui.add(egui::Slider::new(&mut self.game_handler.questions_settings.what_is_this_object.magnitude_cutoff, 0.0..=20.0).text("Star magnitude cutoff"));
        ui.checkbox(&mut self.game_handler.questions_settings.what_is_this_object.replay_incorrect, "Replay incorrectly answered questions");
    }

    fn render_game_settings_guess_the_constellation_subwindow(&mut self, ui: &mut egui::Ui) {
        ui.checkbox(
            &mut self.game_handler.questions_settings.what_constellation_is_this_point_in.show,
            "Show the 'Which constellation is this point in' questions",
        );
        ui.checkbox(
            &mut self.game_handler.questions_settings.what_constellation_is_this_point_in.rotate_to_point,
            "Rotate to the point in question",
        )
        .on_hover_text("Whether or not to rotate the view so that the point in question is in the centre of the screen");
    }

    fn render_game_settings_angular_distance_subwindow(&mut self, ui: &mut egui::Ui) {
        ui.checkbox(&mut self.game_handler.questions_settings.angular_separation.show, "Show the 'What is the angle between..' questions");
        ui.checkbox(&mut self.game_handler.questions_settings.angular_separation.rotate_to_midpoint, "Rotate to the midpoint")
            .on_hover_text("Whether or not to rotate the view so that the point in the middle between the points in question is in the centre of the screen");
    }

    fn render_game_settings_coordinates_subwindow(&mut self, ui: &mut egui::Ui) {
        ui.checkbox(&mut self.game_handler.questions_settings.guess_rad_dec.show, "Show the 'What is the RA/DEC..' questions");
        ui.checkbox(&mut self.game_handler.questions_settings.guess_rad_dec.rotate_to_point, "Rotate to the point in question")
            .on_hover_text("Whether or not to rotate the view so that the point in question is in the centre of the screen");
    }

    fn render_game_settings_magnitude_subwindow(&mut self, ui: &mut egui::Ui) {
        ui.checkbox(&mut self.game_handler.questions_settings.guess_the_magnitude.show, "Show the 'Guess the magnitude' questions");
        ui.checkbox(&mut self.game_handler.questions_settings.guess_the_magnitude.rotate_to_point, "Rotate to the object in question")
            .on_hover_text("Whether or not to rotate the view so that the object in question is in the centre of the screen");
        ui.add(egui::Slider::new(&mut self.game_handler.questions_settings.guess_the_magnitude.magnitude_cutoff, 0.0..=20.0).text("Star magnitude cutoff"));
        ui.checkbox(&mut self.game_handler.questions_settings.guess_the_magnitude.replay_incorrect, "Replay incorrectly answered questions");
    }
}
