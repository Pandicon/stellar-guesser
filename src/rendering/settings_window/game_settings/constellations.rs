use crate::{
    enums,
    Application,
};

impl Application {
    pub fn render_game_settings_constellations_subwindow(&mut self, ui: &mut egui::Ui) {
        let mut any_active_constellation_changed = false;
        egui::CollapsingHeader::new(egui::RichText::new("Use groups").text_style(egui::TextStyle::Body))
            .default_open(true)
            .show(ui, |ui: &mut egui::Ui| {
                let mut groups = self.game_handler.active_constellations_groups.keys().map(|g| g.to_string()).collect::<Vec<String>>();
                groups.sort();
                for group_str in &groups {
                    let entry = self.game_handler.active_constellations_groups.entry(enums::GameLearningStage::from_string(group_str)).or_insert(true);
                    any_active_constellation_changed |= ui.checkbox(entry, group_str).changed();
                }
            });
        egui::CollapsingHeader::new(egui::RichText::new("Constellations groups").text_style(egui::TextStyle::Body))
            .default_open(false)
            .show(ui, |ui| {
                ui.label("Set groups of constellations from which objects should appear in questions");
                egui::ComboBox::from_label("Group to be set")
                    .selected_text(format!("{}", self.state.windows.settings.game_settings.constellation_setting_learning_stage))
                    .show_ui(ui, |ui| {
                        ui.style_mut().wrap = Some(false);
                        ui.selectable_value(
                            &mut self.state.windows.settings.game_settings.constellation_setting_learning_stage,
                            enums::GameLearningStage::NotStarted,
                            format!("{}", enums::GameLearningStage::NotStarted),
                        );
                        ui.selectable_value(
                            &mut self.state.windows.settings.game_settings.constellation_setting_learning_stage,
                            enums::GameLearningStage::Learning,
                            format!("{}", enums::GameLearningStage::Learning),
                        );
                        ui.selectable_value(
                            &mut self.state.windows.settings.game_settings.constellation_setting_learning_stage,
                            enums::GameLearningStage::Reviewing,
                            format!("{}", enums::GameLearningStage::Reviewing),
                        );
                        ui.selectable_value(
                            &mut self.state.windows.settings.game_settings.constellation_setting_learning_stage,
                            enums::GameLearningStage::Learned,
                            format!("{}", enums::GameLearningStage::Learned),
                        );
                    });

                let mut abbreviations = Vec::new();
                for abbreviation in self.game_handler.active_constellations.keys() {
                    abbreviations.push(abbreviation.clone());
                }
                abbreviations.sort();
                for abbreviation in abbreviations {
                    if let Some(constellation) = self.cellestial_sphere.constellations.get(&abbreviation) {
                        if let Some(group_active) = self
                            .game_handler
                            .groups_active_constellations
                            .get_mut(&self.state.windows.settings.game_settings.constellation_setting_learning_stage)
                        {
                            let text = format!("{} ({})", constellation.possible_names[1], constellation.abbreviation);
                            let entry = group_active.entry(abbreviation.clone()).or_insert(true);
                            any_active_constellation_changed |= ui.checkbox(entry, text).changed();
                        }
                    }
                }
            });
        if any_active_constellation_changed {
            for abbreviation in self.cellestial_sphere.constellations.keys() {
                self.game_handler.active_constellations.insert(abbreviation.to_owned(), false);
            }
            for (group, active) in &self.game_handler.active_constellations_groups {
                if *active {
                    if let Some(active_constellations) = self.game_handler.groups_active_constellations.get(group) {
                        for (abbreviation, active) in active_constellations {
                            self.game_handler.active_constellations.entry(abbreviation.to_owned()).and_modify(|v| *v |= *active);
                        }
                    }
                }
            }
        }

        ui.label("Set the constellations from which objects should appear in questions");

        let mut abbreviations = Vec::new();
        for abbreviation in self.game_handler.active_constellations.keys() {
            abbreviations.push(abbreviation.clone());
        }
        abbreviations.sort();
        for abbreviation in abbreviations {
            if let Some(constellation) = self.cellestial_sphere.constellations.get(&abbreviation) {
                let text = format!("{} ({})", constellation.possible_names[1], constellation.abbreviation);
                let entry = self.game_handler.active_constellations.entry(abbreviation.clone()).or_insert(true);
                ui.checkbox(entry, text);
            }
        }
    }
}
