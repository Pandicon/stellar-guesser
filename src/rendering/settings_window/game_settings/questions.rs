use crate::{structs::state::windows::settings::GameSettingsQuestionsSubWindow, Application};

impl Application {
    pub fn render_game_settings_questions_subwindow(&mut self, ui: &mut egui::Ui, tolerance_changed: &mut bool) {
        // TODO: Somehow solve the issue that the row is very wide
        ui.horizontal(|ui| {
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
    }

    fn render_game_settings_find_this_object_subwindow(&mut self, ui: &mut egui::Ui, tolerance_changed: &mut bool) {
        ui.checkbox(&mut self.game_handler.questions_settings.what_is_this_object.show, "Show the 'What is this object' questions");
        ui.checkbox(&mut self.game_handler.questions_settings.find_this_object.show, "Show the 'Find this object' questions");
        ui.checkbox(&mut self.game_handler.questions_settings.find_this_object.show_messiers, "Show Messier objects");
        ui.checkbox(&mut self.game_handler.questions_settings.find_this_object.show_caldwells, "Show Caldwell objects");
        ui.checkbox(&mut self.game_handler.questions_settings.find_this_object.show_ngcs, "Show NGC objects");
        ui.checkbox(&mut self.game_handler.questions_settings.find_this_object.show_ics, "Show IC objects");
        ui.checkbox(&mut self.game_handler.questions_settings.find_this_object.show_bayer, "Show Bayer designations");
        ui.checkbox(&mut self.game_handler.questions_settings.find_this_object.show_starnames, "Show star names");
        self.input.input_field_has_focus |= ui
            .add(egui::Slider::new(&mut self.game_handler.questions_settings.find_this_object.magnitude_cutoff, 0.0..=20.0).text("Star magnitude cutoff"))
            .has_focus();
        let correctness_threshold_widget = ui.add(
            egui::Slider::new(&mut self.game_handler.questions_settings.find_this_object.correctness_threshold, 0.0..=180.0)
                .text("Correctness threshold (degrees)")
                .logarithmic(true),
        );
        self.input.input_field_has_focus |= correctness_threshold_widget.has_focus();
        *tolerance_changed |= correctness_threshold_widget.changed();
        ui.checkbox(&mut self.game_handler.questions_settings.find_this_object.replay_incorrect, "Replay incorrectly answered questions");
    }

    fn render_game_settings_what_is_this_object_subwindow(&mut self, ui: &mut egui::Ui) {
        ui.checkbox(&mut self.game_handler.questions_settings.what_is_this_object.show, "Show the 'What is this object' questions");
        ui.checkbox(&mut self.game_handler.questions_settings.what_is_this_object.show_messiers, "Show Messier objects");
        ui.checkbox(&mut self.game_handler.questions_settings.what_is_this_object.show_caldwells, "Show Caldwell objects");
        ui.checkbox(&mut self.game_handler.questions_settings.what_is_this_object.show_ngcs, "Show NGC objects");
        ui.checkbox(&mut self.game_handler.questions_settings.what_is_this_object.show_ics, "Show IC objects");
        ui.checkbox(&mut self.game_handler.questions_settings.what_is_this_object.show_bayer, "Show Bayer designations");
        ui.checkbox(&mut self.game_handler.questions_settings.what_is_this_object.show_starnames, "Show star names");
        self.input.input_field_has_focus |= ui
            .add(egui::Slider::new(&mut self.game_handler.questions_settings.what_is_this_object.magnitude_cutoff, 0.0..=20.0).text("Star magnitude cutoff"))
            .has_focus();
        ui.checkbox(&mut self.game_handler.questions_settings.what_is_this_object.replay_incorrect, "Replay incorrectly answered questions");
    }

    fn render_game_settings_guess_the_constellation_subwindow(&mut self, ui: &mut egui::Ui) {
        ui.checkbox(
            &mut self.game_handler.questions_settings.what_constellation_is_this_point_in.show,
            "Show the 'Which constellation is this point in' questions",
        );
    }

    fn render_game_settings_angular_distance_subwindow(&mut self, ui: &mut egui::Ui) {
        ui.checkbox(&mut self.game_handler.questions_settings.angular_separation.show, "Show the 'What is the angle between..' questions");
    }

    fn render_game_settings_coordinates_subwindow(&mut self, ui: &mut egui::Ui) {
        ui.checkbox(&mut self.game_handler.questions_settings.guess_rad_dec.show, "Show the 'What is the RA/DEC..' questions");
    }

    fn render_game_settings_magnitude_subwindow(&mut self, ui: &mut egui::Ui) {
        ui.checkbox(&mut self.game_handler.questions_settings.guess_the_magnitude.show, "Show the 'Guess the magnitude' questions");
        self.input.input_field_has_focus |= ui
            .add(egui::Slider::new(&mut self.game_handler.questions_settings.guess_the_magnitude.magnitude_cutoff, 0.0..=20.0).text("Star magnitude cutoff"))
            .has_focus();
        ui.checkbox(&mut self.game_handler.questions_settings.guess_the_magnitude.replay_incorrect, "Replay incorrectly answered questions");
    }
}
