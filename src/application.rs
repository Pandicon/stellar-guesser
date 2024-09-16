use crate::enums::{self, ScreenWidth};
use crate::rendering::caspr::sky_settings;
use crate::rendering::themes::{self, Theme, ThemesHandler};
use crate::{files, public_constants, structs};

use crate::renderer::CellestialSphere;
use crate::structs::graphics_settings;

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
}

impl Application {
    pub fn new(ctx: &egui::Context, authors: String, version: String, storage: &mut Option<crate::storage::Storage>) -> Self {
        egui_extras::install_image_loaders(ctx);

        let mut themes = themes::default_themes();
        let themes_files = files::load_all_files_folder(public_constants::THEMES_FOLDER);
        for file in themes_files {
            if let Err(err) = themes.add_theme_str(&file.content) {
                log::error!("Failed to load a theme (from file {}): {err}", file.name);
            }
        }

        let mut fonts = egui::FontDefinitions::default();
        fonts
            .font_data
            .insert("inter_medium".to_owned(), egui::FontData::from_static(include_bytes!("../assets/fonts/inter/Inter-Medium.otf"))); // .ttf and .otf supported

        // Put the Inter Medium font first (highest priority):
        fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "inter_medium".to_owned());
        ctx.set_fonts(fonts);

        let mut time_spent_start = 0;
        let mut theme = themes::Theme::dark(); // Default in case the restored theme does not exist
        let mut graphics_settings = graphics_settings::GraphicsSettings::default(); // Default in case there are no saved graphics settings
        if let Some(storage) = storage {
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
                        graphics_settings.use_default_star_colour = theme.game_visuals.use_default_star_colour;
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
        }
        ctx.set_visuals(theme.egui_visuals.clone());

        let timestamp = chrono::Utc::now().timestamp();
        let state = state::State::new(timestamp, time_spent_start);

        let mut cellestial_sphere = CellestialSphere::load(storage, &mut theme).unwrap();
        cellestial_sphere.init();
        Self {
            input: input::Input::default(),
            state,

            frame_timestamp: timestamp,
            frame_timestamp_ms: chrono::Utc::now().timestamp_millis(),
            game_handler: GameHandler::init(&mut cellestial_sphere, storage),
            cellestial_sphere,
            frames_handler: FramesHandler::default(),

            graphics_settings,
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
        }
    }

    pub fn update(&mut self, ctx: &egui::Context) {
        #[cfg(any(target_os = "ios", target_os = "android"))]
        // Push the input text restored from key presses to events as a Text event so that input fields take it in by themselves
        ctx.input_mut(|i| i.events.push(egui::Event::Text(self.input.text_from_keys.clone())));
        self.input.input_field_had_focus_last_frame = self.input.input_field_has_focus;
        self.input.input_field_has_focus = false;
        self.frames_handler.current_frame.timestamp_ns = chrono::Local::now().timestamp_nanos_opt().expect("Date out of bounds.");
        self.frame_timestamp = chrono::Utc::now().timestamp();
        self.screen_width = ScreenWidth::from_width(ctx.screen_rect().size().x);
        let cursor_within_central_panel = self.render(ctx);
        self.handle_input(cursor_within_central_panel, ctx);
        self.receive_threads_messages();
        self.frames_handler.handle();
        self.frames_handler.last_frame = chrono::Local::now().timestamp_nanos_opt().expect("Date out of bounds.");
        ctx.request_repaint();
    }

    pub fn save(&mut self, storage: &mut crate::storage::Storage) {
        storage.set_string(
            StorageKeys::TimeSpent.as_ref(),
            (self.state.time_spent_start + (self.frame_timestamp - self.state.start_timestamp)).to_string(),
        );

        let mut inactive_constellations = Vec::new();
        for (abbreviation, value) in &self.game_handler.active_constellations {
            if !*value {
                inactive_constellations.push(abbreviation.as_str());
            }
        }
        storage.set_string(StorageKeys::GameInactiveConstellations.as_ref(), inactive_constellations.join("|"));

        for group in [
            enums::GameLearningStage::NotStarted,
            enums::GameLearningStage::Learning,
            enums::GameLearningStage::Reviewing,
            enums::GameLearningStage::Learned,
        ] {
            if let Some(active_constellations_group) = self.game_handler.groups_active_constellations.get(&group) {
                let mut group_active_constellations = Vec::new();
                for (abbreviation, value) in active_constellations_group {
                    if *value {
                        group_active_constellations.push(abbreviation.as_str());
                    }
                }
                storage.set_string(&format!("{}_{}", StorageKeys::GameInactiveConstellationGroups, group), group_active_constellations.join("|"));
            }
        }

        let mut inactive_constellations_groups = Vec::new();
        for (group, value) in &self.game_handler.active_constellations_groups {
            if !value {
                inactive_constellations_groups.push(group.to_string());
            }
        }
        storage.set_string(StorageKeys::GameInactiveConstellationGroups.as_ref(), inactive_constellations_groups.join("|"));

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

        let now = std::time::Instant::now();
        if now - self.last_state_save_to_disk > self.state_save_to_disk_interval {
            storage.save();
            self.last_state_save_to_disk = now;
        }
    }
}
