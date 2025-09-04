use eframe::egui;

use crate::{files, public_constants, structs::state::windows::settings::ApplicationSettingsSubWindow, Application};

impl Application {
    pub fn render_application_settings_window(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(
                &mut self.state.windows.settings.application_settings.subwindow,
                ApplicationSettingsSubWindow::Input,
                ApplicationSettingsSubWindow::Input.as_ref(),
            );
            ui.selectable_value(
                &mut self.state.windows.settings.application_settings.subwindow,
                ApplicationSettingsSubWindow::Theme,
                ApplicationSettingsSubWindow::Theme.as_ref(),
            );
        });
        ui.separator();
        egui::ScrollArea::vertical()
            .auto_shrink([false, true])
            .show(ui, |ui| match self.state.windows.settings.application_settings.subwindow {
                ApplicationSettingsSubWindow::Input => self.render_application_settings_input_subwindow(ui),
                ApplicationSettingsSubWindow::Theme => self.render_application_settings_theme_subwindow(ctx, ui),
            });
    }

    pub fn render_application_settings_input_subwindow(&mut self, ui: &mut egui::Ui) {
        #[cfg(target_os = "android")]
        let previous_display_onscreen_keyboard = self.input.settings.display_onscreen_keyboard;

        ui.heading("Input method");
        ui.checkbox(&mut self.input.settings.display_onscreen_keyboard, "Use on-screen keyboard");
        ui.label("You may use this keyboard as a replacement to the default input method provided by your system, both on mobile and desktop.");
        ui.label("It is known that some devices experience issues with the native keyboard on Android. To get around this, you may choose to use an alternative keyboard built into this application, which does not experience those issues. However, it will be different to what you are used to, so it is up to each user to decide. To help you make the correct choice, please consider and test out the following:\n - If you only want to play the game and answer questions, you will most likely only need the letters and numbers. You can try typing them into the text box below to see if they work as expected.\n - If you want to also edit questions packs or do other \"advanced\" things, you may also need some special characters. The exact set will depend on your use case, so you may find out that some characters are misbehaving later. For now, you can try typing in the following set of characters: \"():,.'");
        ui.text_edit_singleline(&mut self.state.windows.settings.application_settings.test_input);

        #[cfg(target_os = "android")]
        if !previous_display_onscreen_keyboard && self.input.settings.display_onscreen_keyboard {
            crate::show_soft_input(false);
        }
    }

    pub fn render_application_settings_theme_subwindow(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        let previous_theme_name = self.theme.name.clone();
        let mut selected_theme_name = self.theme.name.clone();
        ui.label("Theme: ");
        egui::ComboBox::from_id_salt("Theme: ").selected_text(&self.theme.name).show_ui(ui, |ui| {
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
            let mut themes = self.themes.themes_names().collect::<Vec<&String>>();
            themes.sort();
            for theme_name in themes {
                ui.selectable_value(&mut selected_theme_name, theme_name.to_owned(), theme_name);
            }
        });
        if selected_theme_name != previous_theme_name {
            match self.themes.get(&selected_theme_name) {
                Some(theme) => {
                    self.apply_theme(ctx, theme.clone());
                }
                None => log::error!("Failed to get the selected theme: {selected_theme_name}"),
            }
        }
        ui.heading("Export theme");
        ui.label("Export the current settings into a theme");
        ui.horizontal(|ui| {
            ui.label("Theme name: ");
            ui.text_edit_singleline(&mut self.theme.name);
        });
        if ui.button("Export").clicked() {
            if let Some(path) = files::get_dir_opt(public_constants::THEMES_FOLDER) {
                #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
                let save_path_opt: Option<std::path::PathBuf> = {
                    let dialog = rfd::FileDialog::new().add_filter("Theme", &["json"]).set_directory(path);
                    dialog.save_file()
                };
                #[cfg(any(target_os = "android", target_os = "ios"))]
                let save_path_opt: Option<std::path::PathBuf> = {
                    let mut save_path_intermediate = path;
                    save_path_intermediate.push(format!("{}--{}.json", &self.theme.name, chrono::Local::now().timestamp_millis()));
                    Some(save_path_intermediate)
                };
                match save_path_opt {
                    Some(save_path) => match serde_json::to_string_pretty(&self.theme) {
                        Ok(theme_to_save) => {
                            if let Some(dir) = save_path.parent() {
                                if !dir.exists() {
                                    if let Err(err) = std::fs::create_dir_all(dir) {
                                        log::error!("Failed to create the folders for the theme: {err}");
                                    }
                                }
                            } else {
                                log::warn!("No theme folder: {:?}", save_path);
                            }
                            if let Err(err) = std::fs::write(save_path, theme_to_save) {
                                log::error!("Failed to save the theme: {err}");
                            } else {
                                self.themes.insert(self.theme.name.clone(), self.theme.clone());
                            }
                        }
                        Err(err) => log::error!("Failed to serialize the theme: {err}"),
                    },
                    None => log::info!("Theme saving cancelled by the user"),
                }
            }
        }
    }
}
