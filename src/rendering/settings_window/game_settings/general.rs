use crate::{
    enums::GameStage,
    Application,
};

impl Application {
    pub fn render_game_settings_general_subwindow(&mut self, ui: &mut egui::Ui) {
        ui.checkbox(&mut self.game_handler.game_settings.is_scored_mode, "Play in scored mode?");
        self.input.input_field_has_focus |= ui
            .add(
                egui::Slider::new(&mut self.game_handler.game_settings.no_of_questions, 1..=self.game_handler.possible_no_of_questions)
                    .text("Number of questions")
                    .logarithmic(true),
            )
            .has_focus();
        if ui.button("Reset").clicked() {
            self.game_handler.stage = GameStage::NotStartedYet;
            self.game_handler.reset_used_questions(&mut self.cellestial_sphere);

            // Remove all game markers from the screen
            self.cellestial_sphere.deinit_single_renderer("markers", "game");
            // Disable adding a game marker on click
            self.game_handler.add_marker_on_click = false;
        }
    }
}
