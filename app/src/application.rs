use eframe::egui;

use crate::enums::ScreenWidth;
use crate::rendering::caspr::sky_settings;
use crate::rendering::initial_setup;
use crate::rendering::themes::{self, Theme, ThemesHandler};
use crate::{files, public_constants, server_communication, structs};

use crate::renderer::CellestialSphere;
use crate::structs::{graphics_settings, testing_settings};

use self::frames_handler::FramesHandler;

use crate::game::game_handler::{self, GameHandler};

use crate::{
    enums::StorageKeys,
    input,
    structs::{
        frames_handler,
        state::{self, threads_communication},
    },
};

pub struct Application {
    pub input: input::Input,
    pub state: state::State,

    pub frame_timestamp: i64,
    pub frame_timestamp_ms: i64,
    pub cellestial_sphere: CellestialSphere,
    pub frames_handler: FramesHandler,
    pub game_handler: game_handler::GameHandler,

    pub graphics_settings: graphics_settings::GraphicsSettings,
    pub testing_settings: testing_settings::TestingSettings,
    pub theme: Theme,
    pub themes: ThemesHandler,

    pub authors: String,
    pub version: structs::version_information::VersionInformation,

    pub last_state_save: std::time::Instant,
    pub last_state_save_to_disk: std::time::Instant,
    pub state_save_interval: std::time::Duration,
    pub state_save_to_disk_interval: std::time::Duration,

    pub screen_width: ScreenWidth,

    pub threads_communication: threads_communication::ThreadsCommunication,

    pub toasts: egui_notify::Toasts,

    pub testing_mode: bool,

    pub onscreen_keyboard: egui_keyboard::Keyboard,

    pub initial_setup_stage: initial_setup::InitialSetupStage,
}

impl Application {
    pub fn new(cc: &eframe::CreationContext<'_>, authors: String, version: String) -> Self {
        let ctx = &cc.egui_ctx;
        egui_extras::install_image_loaders(ctx);

        let testing_mode = std::env::var("TESTING").unwrap_or_default().to_lowercase() == *"true";

        let mut themes = themes::default_themes();
        let themes_files = files::load_all_files_folder(public_constants::THEMES_FOLDER);
        for file in themes_files {
            if let Err(err) = themes.add_theme_str(&file.content) {
                log::error!("Failed to load a theme (from file {}): {err}", file.name);
            }
        }

        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "inter_medium".to_owned(),
            std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/fonts/inter/Inter-Medium.otf"))),
        ); // .ttf and .otf supported

        // Put the Inter Medium font first (highest priority):
        fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "inter_medium".to_owned());
        ctx.set_fonts(fonts);

        let mut time_spent_start = 0;
        let mut theme = themes::Theme::dark(); // Default in case the restored theme does not exist
        let mut graphics_settings = graphics_settings::GraphicsSettings::default(); // Default in case there are no saved graphics settings
        let mut input = input::Input::default();
        let mut initial_setup_stage = initial_setup::InitialSetupStage::default();
        if let Some(storage) = cc.storage {
            if let Some(time_spent_restore) = storage.get_string(StorageKeys::TimeSpent.as_ref()) {
                match time_spent_restore.parse() {
                    Ok(time_spent) => time_spent_start = time_spent,
                    Err(err) => log::error!("Failed to parse the time spent: {err}"),
                }
            }
            if let Some(theme_str) = storage.get_string(StorageKeys::Theme.as_ref()) {
                match serde_json::from_str(&theme_str) {
                    Ok(theme_loaded) => {
                        theme = theme_loaded;
                        if let Some(same_name_theme) = themes.get(&theme.name) {
                            if same_name_theme != &theme {
                                theme.name += " (restored)";
                            }
                        }
                        // IMPORTANT: Due to this setting, the theme has to be loaded before the graphics settings, else they will always be overwritten by the theme default
                        graphics_settings.use_overriden_star_colour = theme.game_visuals.use_overriden_star_colour;
                    }
                    Err(err) => log::error!("Failed to deserialize the theme: {err}"),
                }
            }
            if let Some(graphics_settings_str) = storage.get_string(StorageKeys::GraphicsSettings.as_ref()) {
                match serde_json::from_str(&graphics_settings_str) {
                    Ok(graphics_settings_loaded) => graphics_settings = graphics_settings_loaded,
                    Err(err) => log::error!("Failed to deserialize the graphics settings: {err}"),
                }
            }
            if let Some(input_settings_str) = storage.get_string(StorageKeys::InputSettings.as_ref()) {
                match serde_json::from_str(&input_settings_str) {
                    Ok(input_settings_loaded) => input.settings = input_settings_loaded,
                    Err(err) => log::error!("Failed to deserialize the input settings: {err}"),
                }
            }
            if let Some(initial_setup_stage_str) = storage.get_string(StorageKeys::InitialSetupStage.as_ref()) {
                match serde_json::from_str(&initial_setup_stage_str) {
                    Ok(initial_setup_stage_loaded) => initial_setup_stage = initial_setup_stage_loaded,
                    Err(err) => log::error!("Failed to deserialize the state of showing initial setup: {err}"),
                }
            }
        }
        let first_application_launch = time_spent_start == 0;
        let timestamp = chrono::Utc::now().timestamp();
        let mut state = state::State::new(timestamp, time_spent_start);
        if let Some(storage) = cc.storage {
            if let Some(last_question_pack_query) = storage.get_string(StorageKeys::QuestionPackQuery.as_ref()) {
                state.windows.settings.game_settings.query = last_question_pack_query;
            }
            if let Some(last_question_pack_description) = storage.get_string(StorageKeys::QuestionPackDescription.as_ref()) {
                state.windows.settings.game_settings.question_pack_new_description = last_question_pack_description;
            }
        }
        ctx.set_visuals(theme.egui_visuals.clone());

        let mut cellestial_sphere = CellestialSphere::load(cc.storage, &mut theme).unwrap();
        cellestial_sphere.init();
        let game_handler = GameHandler::init(&mut cellestial_sphere, cc.storage, first_application_launch);
        if game_handler.question_packs.contains_key(&game_handler.active_question_pack) {
            state.windows.settings.game_settings.question_pack_new_name = game_handler.active_question_pack.clone();
        }
        let mut app = Self {
            input,
            state,

            frame_timestamp: timestamp,
            frame_timestamp_ms: chrono::Utc::now().timestamp_millis(),
            game_handler,
            cellestial_sphere,
            frames_handler: FramesHandler::default(),

            graphics_settings,
            testing_settings: testing_settings::TestingSettings::default(),
            theme,
            themes,

            authors,
            version: structs::version_information::VersionInformation::only_current(version),

            last_state_save: std::time::Instant::now(),
            last_state_save_to_disk: std::time::Instant::now(),
            state_save_interval: std::time::Duration::from_secs(5),
            state_save_to_disk_interval: std::time::Duration::from_secs(60),

            screen_width: ScreenWidth::from_width(ctx.screen_rect().size().x),

            threads_communication: threads_communication::ThreadsCommunication::default(),

            toasts: egui_notify::Toasts::default().with_anchor(egui_notify::Anchor::BottomRight),

            testing_mode,

            onscreen_keyboard: egui_keyboard::Keyboard::new(['⬆', '⇧'], '⌫'),

            initial_setup_stage,
        };
        server_communication::check_for_updates::check_for_updates(
            &mut app.threads_communication,
            crate::PLATFORM,
            crate::VERSION,
            threads_communication::CheckUpdatesShowPopup::OnFoundUpdate,
        );
        app
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.input.settings.display_onscreen_keyboard {
            self.onscreen_keyboard.pump_events(ctx);
        }
        #[cfg(any(target_os = "ios", target_os = "android"))]
        // Push the input text restored from key presses to events as a Text event so that input fields take it in by themselves
        ctx.input_mut(|i| i.events.push(egui::Event::Text(self.input.text_from_keys.clone())));
        self.frames_handler.current_frame.timestamp_ns = chrono::Local::now().timestamp_nanos_opt().expect("Date out of bounds.");
        self.frame_timestamp = chrono::Utc::now().timestamp();
        self.screen_width = ScreenWidth::from_width(ctx.screen_rect().size().x);
        let cursor_within_central_panel = self.render(ctx);
        self.handle_input(cursor_within_central_panel, ctx);
        self.receive_threads_messages();
        self.toasts.show(ctx);
        self.frames_handler.handle();
        self.frames_handler.last_frame = chrono::Local::now().timestamp_nanos_opt().expect("Date out of bounds.");
        if self.game_handler.switch_to_next_part {
            let data = game_handler::QuestionCheckingData {
                cellestial_sphere: &mut self.cellestial_sphere,
                theme: &self.theme,
                game_stage: &mut self.game_handler.stage,
                score: &mut self.game_handler.score,
                possible_score: &mut self.game_handler.possible_score,
                is_scored_mode: self.game_handler.game_settings.is_scored_mode,
                current_question: self.game_handler.current_question,
                used_questions: &mut self.game_handler.used_questions,
                add_marker_on_click: &mut self.game_handler.add_marker_on_click,
                questions_settings: &self.game_handler.questions_settings,
                question_number: &mut self.game_handler.question_number,
                start_next_question: &mut self.game_handler.switch_to_next_question,
                switch_to_next_part: &mut self.game_handler.switch_to_next_part,
            };
            self.game_handler.question_catalog[self.game_handler.current_question].generic_to_next_part(data);
            self.game_handler.switch_to_next_part = false;
        }
        if self.game_handler.switch_to_next_question {
            self.game_handler.next_question(&mut self.cellestial_sphere, &self.theme);
            self.game_handler.switch_to_next_question = false;
        }

        let input_field_has_focus = ctx.wants_keyboard_input();
        if self.input.settings.display_onscreen_keyboard {
            self.onscreen_keyboard.show(ctx);
        } else {
            // Toggle software keyboard
            #[cfg(target_os = "android")]
            if input_field_has_focus && !self.input.input_field_had_focus_last_frame {
                // There was no focus on any text input field last frame, but there is this frame -> show the keyboard
                crate::show_soft_input(true);
            } else if !input_field_has_focus && self.input.input_field_had_focus_last_frame {
                // There was focus on some text input field last frame, but there is not this frame -> hide the keyboard
                crate::show_soft_input(false);
            }
        }
        self.input.input_field_had_focus_last_frame = input_field_has_focus;

        ctx.request_repaint();
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        storage.set_string(
            StorageKeys::TimeSpent.as_ref(),
            (self.state.time_spent_start + (self.frame_timestamp - self.state.start_timestamp)).to_string(),
        );

        match serde_json::to_string(&self.game_handler.questions_settings) {
            Ok(string) => storage.set_string(StorageKeys::GameQuestionSettings.as_ref(), string),
            Err(err) => log::error!("Failed to serialize game question settings: {:?}", err),
        }

        match serde_json::to_string(&self.game_handler.game_settings) {
            Ok(string) => storage.set_string(StorageKeys::GameSettings.as_ref(), string),
            Err(err) => log::error!("Failed to serialize game settings: {:?}", err),
        }

        match serde_json::to_string(&sky_settings::SkySettingsRaw::from_sky_settings(&self.cellestial_sphere.sky_settings)) {
            Ok(string) => storage.set_string(StorageKeys::SkySettings.as_ref(), string),
            Err(err) => log::error!("Failed to serialize sky settings: {:?}", err),
        }

        match serde_json::to_string(&self.theme) {
            Ok(string) => storage.set_string(StorageKeys::Theme.as_ref(), string),
            Err(err) => log::error!("Failed to serialize the theme: {:?}", err),
        }

        match serde_json::to_string(&self.graphics_settings) {
            Ok(string) => storage.set_string(StorageKeys::GraphicsSettings.as_ref(), string),
            Err(err) => log::error!("Failed to serialize graphics settings: {:?}", err),
        }

        match serde_json::to_string(&self.input.settings) {
            Ok(string) => storage.set_string(StorageKeys::InputSettings.as_ref(), string),
            Err(err) => log::error!("Failed to serialize input settings: {:?}", err),
        }

        match serde_json::to_string(&self.initial_setup_stage) {
            Ok(string) => storage.set_string(StorageKeys::InitialSetupStage.as_ref(), string),
            Err(err) => log::error!("Failed to serialize the initial setup stage: {:?}", err),
        }

        let question_packs = self
            .game_handler
            .question_packs
            .iter()
            .filter(|(_, pack)| pack.file_path.is_none()) // Do not save question packs that are in separate files
            .map(|(name, pack)| crate::game::questions::question_pack_to_string(name, pack))
            .collect::<Vec<String>>()
            .join(crate::game::game_handler::QUESTION_PACKS_DIV);
        storage.set_string(StorageKeys::QuestionPacks.as_ref(), question_packs);
        storage.set_string(StorageKeys::ActiveQuestionPack.as_ref(), self.game_handler.active_question_pack.clone());
        storage.set_string(StorageKeys::QuestionPackQuery.as_ref(), self.state.windows.settings.game_settings.internal_query.clone());
        storage.set_string(
            StorageKeys::QuestionPackDescription.as_ref(),
            self.state.windows.settings.game_settings.question_pack_new_description.clone(),
        );

        self.game_handler.constellation_groups_settings.save_to_storage(storage);
    }
}
