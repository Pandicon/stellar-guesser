use crate::{
    public_constants,
    structs::state::windows::settings::{GameSettingsQuestionsSubWindow, GameSettingsType},
    Application,
};
use angle::Angle;
use eframe::egui;

impl Application {
    pub fn render_game_settings_questions_subwindow(&mut self, ui: &mut egui::Ui, tolerance_changed: &mut bool) {
        let prev_active_pack = self.game_handler.active_question_pack.clone();
        ui.horizontal(|ui| {
            eframe::egui::ComboBox::from_label("Select question pack")
                .selected_text(&self.game_handler.active_question_pack)
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap_mode = Some(eframe::egui::TextWrapMode::Extend);
                    let mut packs = self.game_handler.question_packs.keys().cloned().collect::<Vec<String>>();
                    packs.sort_by_key(|a| a.to_lowercase());
                    for pack_name in packs {
                        ui.selectable_value(&mut self.game_handler.active_question_pack, pack_name.clone(), &pack_name)
                            .on_hover_text(self.game_handler.question_packs.get(&pack_name).map(|pack| pack.description.as_str()).unwrap_or(""));
                    }
                });
            let removed_group = ui
                .add_enabled_ui(self.game_handler.question_packs.keys().len() != 0, |ui| {
                    if ui.button("Remove pack").clicked() {
                        if let Some(pack) = self.game_handler.question_packs.get(&self.game_handler.active_question_pack) {
                            let remove = if let Some(path) = &pack.file_path {
                                let path_buf = std::path::PathBuf::from(path);
                                if let Err(err) = std::fs::remove_file(&path_buf) {
                                    let exists_res = std::fs::exists(&path_buf);
                                    log::error!(
                                        "Failed to remove the question pack file: {err} (file path: {path:?}, file path buf: {path_buf:?}). {}",
                                        match exists_res {
                                            Ok(exists) =>
                                                if exists {
                                                    "It is known the file exists."
                                                } else {
                                                    "It is known the file does not exist."
                                                },
                                            Err(ref err) => {
                                                log::error!("Could not check if the question pack exists: {err}");
                                                "It is not known if the file exists."
                                            }
                                        }
                                    );
                                    !exists_res.unwrap_or(true)
                                } else {
                                    true
                                }
                            } else {
                                true
                            };
                            if remove {
                                self.game_handler.question_packs.remove(&self.game_handler.active_question_pack);
                                self.game_handler.active_question_pack = String::new();
                                true
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                })
                .inner;
            if !removed_group && self.game_handler.active_question_pack != prev_active_pack {
                self.state.windows.settings.game_settings.question_pack_new_name = self.game_handler.active_question_pack.clone();
                self.state.windows.settings.game_settings.settings_type = GameSettingsType::Advanced;
                self.state.windows.settings.game_settings.query = self.game_handler.question_packs.get(&self.game_handler.active_question_pack).unwrap().query.clone();
                self.state.windows.settings.game_settings.question_pack_new_description = self.game_handler.question_packs.get(&self.game_handler.active_question_pack).unwrap().description.clone();
                let new_questions = self
                    .cellestial_sphere
                    .generate_questions(&self.game_handler.question_packs.get(&self.game_handler.active_question_pack).unwrap().question_objects);
                self.game_handler.possible_no_of_questions = new_questions.len() as u32;
                self.game_handler.question_catalog = new_questions;
                self.game_handler.reset_used_questions(&mut self.cellestial_sphere);
                self.game_handler.current_question = 0;
                self.game_handler.stage = crate::enums::GameStage::NotStartedYet;
                self.game_handler.question_number_text = String::new();
            }
        });
        ui.horizontal(|ui| {
            ui.label("Question pack name");
            ui.text_edit_singleline(&mut self.state.windows.settings.game_settings.question_pack_new_name);
            self.state.windows.settings.game_settings.question_pack_new_name = self
                .state
                .windows
                .settings
                .game_settings
                .question_pack_new_name
                .replace(crate::game::game_handler::QUESTION_PACKS_DIV, "")
                .replace(crate::game::game_handler::QUESTION_PACK_PARTS_DIV, "")
                .replace(crate::game::game_handler::QUESTION_PACK_QUESTIONS_DIV, "")
                .replace(crate::game::game_handler::QUESTION_PACK_QUESTIONS_PARTS_DIV, "");
        });
        ui.label("Question pack description");
        ui.add(eframe::egui::TextEdit::multiline(&mut self.state.windows.settings.game_settings.question_pack_new_description).desired_rows(2));
        self.state.windows.settings.game_settings.question_pack_new_description = self
            .state
            .windows
            .settings
            .game_settings
            .question_pack_new_description
            .replace(crate::game::game_handler::QUESTION_PACKS_DIV, "")
            .replace(crate::game::game_handler::QUESTION_PACK_PARTS_DIV, "")
            .replace(crate::game::game_handler::QUESTION_PACK_QUESTIONS_DIV, "")
            .replace(crate::game::game_handler::QUESTION_PACK_QUESTIONS_PARTS_DIV, "");
        ui.separator();

        let mut can_evaluate = true;
        let mut settings_all = Vec::new();
        ui.collapsing("Edit question pack", |ui| {
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
                    self.state.windows.settings.game_settings.internal_query = self.state.windows.settings.game_settings.generated_query.clone();
                }
                GameSettingsType::Advanced => {
                    ui.label("Enter the questions query here:");
                    ui.add(egui::TextEdit::multiline(&mut self.state.windows.settings.game_settings.query).desired_width(f32::INFINITY));
                    self.state.windows.settings.game_settings.internal_query = self.state.windows.settings.game_settings.query.clone();
                }
            }
            ui.separator();
            let mut text_parts = Vec::new();
            for line in self.state.windows.settings.game_settings.internal_query.split('\n') {
                let no_spaces = line.replace(" ", "");
                let mut spl = no_spaces.split("):").map(|s| s.trim()).filter(|s| !s.is_empty()).collect::<Vec<&str>>();
                if spl.is_empty() {
                    continue;
                }
                let (parsed_result, ast_res) = if spl.len() > 1 {
                    let query = spl.pop().unwrap(); //.replace(":", "");
                    match crate::game::questions_filter::parser::Parser::new(query).parse(&self.game_handler.constellation_groups_settings.constellation_groups) {
                        Ok(Some(crate::game::questions_filter::parser::Node::Keyword(ast))) => (format!("{:?}", ast), Ok(Some(ast))),
                        Ok(Some(crate::game::questions_filter::parser::Node::Value(_))) | Ok(None) => (String::from("No restrictions"), Ok(None)),
                        Err(err) => {
                            can_evaluate = false;
                            (format!("Error when parsing the query: {err}"), Err(""))
                        }
                    }
                } else {
                    (String::from("No restrictions"), Ok(None))
                };
                let mut joined = spl.join("");
                if joined.trim().ends_with(")") {
                    joined = String::from(joined.trim());
                    joined.pop();
                }
                let spl = joined.split('(').map(|s| s.trim()).filter(|s| !s.is_empty()).collect::<Vec<&str>>();
                if spl.len() < 2 {
                    continue;
                }
                let question_type = spl[0];
                let question_settings = spl[1];
                let question_type = crate::game::questions_filter::parser::parse_question_type_and_settings(question_type, question_settings);
                let question_type_res = match question_type {
                    Ok(question_type) => {
                        let res = format!("{question_type:?}");
                        if let Ok(ast_opt) = ast_res {
                            settings_all.push((ast_opt, question_type));
                        }
                        res
                    }
                    Err(err) => {
                        can_evaluate = false;
                        err
                    }
                };
                text_parts.push(format!("{question_type_res}: {parsed_result}"));
            }
            let joined = text_parts.join("\n");
            let replaced = joined.replace("SmallSettings {", "{");
            ui.label("Parsed query:");
            ui.label(egui::RichText::new(replaced).code());
        });
        ui.add_enabled_ui(can_evaluate, |ui| {
            ui.horizontal(|ui| {
                let save_button = if self.game_handler.question_packs.contains_key(&self.state.windows.settings.game_settings.question_pack_new_name) {
                    ui.button("Evaluate and save")
                } else {
                    ui.button("Evaluate and create new pack")
                };
                if save_button.clicked() {
                    let res = self.cellestial_sphere.evaluate_questions_query(&settings_all);
                    self.game_handler.question_packs.insert(
                        self.state.windows.settings.game_settings.question_pack_new_name.clone(),
                        crate::game::questions_filter::QuestionPack {
                            question_objects: res,
                            query: self.state.windows.settings.game_settings.internal_query.clone(),
                            description: self.state.windows.settings.game_settings.question_pack_new_description.clone(),
                            file_path: None,
                        },
                    );
                    self.game_handler.active_question_pack = self.state.windows.settings.game_settings.question_pack_new_name.clone();
                }
                let export_button = ui.button("Evaluate and export");
                if export_button.clicked() {
                    let res = self.cellestial_sphere.evaluate_questions_query(&settings_all);
                    if let Some(path) = crate::files::get_dir_opt(public_constants::QUESTION_PACKS_FOLDER) {
                        log::debug!("Question pack save path: {:?}", path);
                        let name = self.state.windows.settings.game_settings.question_pack_new_name.clone();
                        let pack = crate::game::questions_filter::QuestionPack {
                            question_objects: res,
                            query: self.state.windows.settings.game_settings.internal_query.clone(),
                            description: self.state.windows.settings.game_settings.question_pack_new_description.clone(),
                            file_path: None,
                        };
                        let pack_string = crate::game::questions::question_pack_to_string(&name, &pack);
                        #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
                        let save_path_opt: Option<std::path::PathBuf> = {
                            if !path.exists() {
                                if let Err(err) = std::fs::create_dir_all(&path) {
                                    log::error!("Failed to create the question pack folder: {err}");
                                } else {
                                    log::debug!("Created the folder for question packs")
                                }
                            }
                            let dialog = rfd::FileDialog::new().add_filter("Question pack", &["txt"]).set_directory(path);
                            dialog.save_file()
                        };
                        #[cfg(any(target_os = "android", target_os = "ios"))]
                        let save_path_opt: Option<std::path::PathBuf> = {
                            let mut save_path_intermediate = path;
                            save_path_intermediate.push(format!("{}--{}.txt", &name, chrono::Local::now().timestamp_millis()));
                            Some(save_path_intermediate)
                        };
                        match save_path_opt {
                            Some(save_path) => {
                                if let Some(dir) = save_path.parent() {
                                    if !dir.exists() {
                                        if let Err(err) = std::fs::create_dir_all(dir) {
                                            log::error!("Failed to create the folders for the question pack: {err}");
                                        } else {
                                            log::debug!("Created the folder for question packs")
                                        }
                                    }
                                } else {
                                    log::warn!("No question pack folder: {:?}", save_path);
                                }
                                if let Err(err) = std::fs::write(save_path, pack_string) {
                                    log::error!("Failed to save the question pack: {err}");
                                }
                            }
                            None => log::info!("Question pack saving cancelled by the user"),
                        }
                    }
                }
                if save_button.clicked() || export_button.clicked() {
                    let new_questions = self
                        .cellestial_sphere
                        .generate_questions(&self.game_handler.question_packs.get(&self.game_handler.active_question_pack).unwrap().question_objects);
                    self.game_handler.possible_no_of_questions = new_questions.len() as u32;
                    self.game_handler.question_catalog = new_questions;
                    self.game_handler.reset_used_questions(&mut self.cellestial_sphere);
                    self.game_handler.current_question = 0;
                    self.game_handler.stage = crate::enums::GameStage::NotStartedYet;
                    self.game_handler.question_number_text = String::new();
                }
            });
        });
    }

    fn generate_query_from_basic(&self) -> String {
        use crate::game::questions;

        let mut query_parts = Vec::new();
        if self.game_handler.questions_settings.find_this_object.show {
            let mut question_settings = questions::find_this_object::SmallSettings {
                correctness_threshold: *self.game_handler.questions_settings.find_this_object.correctness_threshold.to_deg().as_value(),
                rotate_to_answer: self.game_handler.questions_settings.find_this_object.rotate_to_correct_point,
                replay_incorrect: self.game_handler.questions_settings.find_this_object.replay_incorrect,
                ask_messier: false,
                ask_caldwell: false,
                ask_ic: false,
                ask_ngc: false,
                ask_hd: false,
                ask_hip: false,
                ask_bayer: false,
                ask_flamsteed: false,
                ask_proper: false,
            };
            let mut settings_catalogues = Vec::new();
            if self.game_handler.questions_settings.find_this_object.show_messiers {
                settings_catalogues.push("CATALOGUE(MESSIER)");
                question_settings.ask_messier = true;
            }
            if self.game_handler.questions_settings.find_this_object.show_caldwells {
                settings_catalogues.push("CATALOGUE(CALDWELL)");
                question_settings.ask_caldwell = true;
            }
            if self.game_handler.questions_settings.find_this_object.show_ngcs {
                settings_catalogues.push("CATALOGUE(NGC)");
                question_settings.ask_ngc = true;
            }
            if self.game_handler.questions_settings.find_this_object.show_ics {
                settings_catalogues.push("CATALOGUE(IC)");
                question_settings.ask_ic = true;
            }
            if self.game_handler.questions_settings.find_this_object.show_bayer {
                settings_catalogues.push("CATALOGUE(BAYER)");
                question_settings.ask_bayer = true;
            }
            if self.game_handler.questions_settings.find_this_object.show_starnames {
                settings_catalogues.push("AND(TYPE(STAR), CATALOGUE(PROPER_NAME))");
                question_settings.ask_proper = true;
            }
            if !settings_catalogues.is_empty() {
                let settings = format!(
                    "OR(AND(TYPE(STAR), MAG_BELOW({}), OR({})), AND(NOT(TYPE(STAR)), OR({})))",
                    self.game_handler.questions_settings.find_this_object.magnitude_cutoff,
                    settings_catalogues.join(", "),
                    settings_catalogues.join(", ")
                );
                if let Ok(question_settings) = serde_json::to_string(&question_settings) {
                    query_parts.push(format!("FIND_THIS_OBJECT({}): {}", question_settings, settings));
                }
            };
        }
        if self.game_handler.questions_settings.what_is_this_object.show {
            let question_settings = questions::which_object_is_here::SmallSettings {
                rotate_to_point: self.game_handler.questions_settings.what_is_this_object.rotate_to_point,
                replay_incorrect: self.game_handler.questions_settings.what_is_this_object.replay_incorrect,
                accept_messier: true,
                accept_caldwell: true,
                accept_ngc: true,
                accept_ic: true,
                accept_hip: true,
                accept_hd: true,
                accept_proper: true,
                accept_bayer: true,
                accept_flamsteed: true,
            };
            let mut settings_catalogues = Vec::new();
            if self.game_handler.questions_settings.what_is_this_object.show_messiers {
                settings_catalogues.push("CATALOGUE(MESSIER)");
            }
            if self.game_handler.questions_settings.what_is_this_object.show_caldwells {
                settings_catalogues.push("CATALOGUE(CALDWELL)");
            }
            if self.game_handler.questions_settings.what_is_this_object.show_ngcs {
                settings_catalogues.push("CATALOGUE(NGC)");
            }
            if self.game_handler.questions_settings.what_is_this_object.show_ics {
                settings_catalogues.push("CATALOGUE(IC)");
            }
            if self.game_handler.questions_settings.what_is_this_object.show_bayer {
                settings_catalogues.push("CATALOGUE(BAYER)");
            }
            if self.game_handler.questions_settings.what_is_this_object.show_starnames {
                settings_catalogues.push("AND(TYPE(STAR), CATALOGUE(PROPER_NAME))");
            }
            if !settings_catalogues.is_empty() {
                let settings = format!(
                    "OR(AND(TYPE(STAR), MAG_BELOW({}), OR({})), AND(NOT(TYPE(STAR)), OR({})))",
                    self.game_handler.questions_settings.what_is_this_object.magnitude_cutoff,
                    settings_catalogues.join(", "),
                    settings_catalogues.join(", ")
                );
                if let Ok(question_settings) = serde_json::to_string(&question_settings) {
                    query_parts.push(format!("WHAT_IS_THIS_OBJECT({}): {}", question_settings, settings));
                }
            };
        }
        if self.game_handler.questions_settings.what_constellation_is_this_point_in.show {
            let question_settings = questions::which_constellation_is_point_in::SmallSettings {
                rotate_to_point: self.game_handler.questions_settings.what_constellation_is_this_point_in.rotate_to_point,
            };
            if let Ok(question_settings) = serde_json::to_string(&question_settings) {
                query_parts.push(format!("WHICH_CONSTELLATION_IS_THIS_POINT_IN({})", question_settings));
            }
        }
        if self.game_handler.questions_settings.angular_separation.show {
            let question_settings = questions::angular_separation::SmallSettings {
                rotate_to_midpoint: self.game_handler.questions_settings.angular_separation.rotate_to_midpoint,
            };
            if let Ok(question_settings) = serde_json::to_string(&question_settings) {
                query_parts.push(format!("ANGULAR_SEPARATION({})", question_settings));
            }
        }
        if self.game_handler.questions_settings.guess_rad_dec.show {
            let question_settings = questions::guess_ra_dec::SmallSettings {
                rotate_to_point: self.game_handler.questions_settings.guess_rad_dec.rotate_to_point,
            };
            if let Ok(question_settings) = serde_json::to_string(&question_settings) {
                query_parts.push(format!("GUESS_DEC({})", question_settings));
                query_parts.push(format!("GUESS_RA({})", question_settings));
            }
        }
        if self.game_handler.questions_settings.guess_the_magnitude.show {
            let question_settings = questions::guess_the_magnitude::SmallSettings {
                rotate_to_point: self.game_handler.questions_settings.guess_the_magnitude.rotate_to_point,
                replay_incorrect: self.game_handler.questions_settings.guess_the_magnitude.replay_incorrect,
            };
            if let Ok(question_settings) = serde_json::to_string(&question_settings) {
                query_parts.push(format!(
                    "GUESS_THE_MAGNITUDE({}): MAG_BELOW({})",
                    question_settings, self.game_handler.questions_settings.guess_the_magnitude.magnitude_cutoff
                ));
            }
        }
        let query = query_parts.join("\n");
        query.replace("SmallSettings {", "{")
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
