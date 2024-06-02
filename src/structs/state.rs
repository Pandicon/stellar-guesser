use crate::enums;

pub struct State {
    pub start_timestamp: i64,
    pub time_spent_start: i64,

    pub windows: WindowsState,
}

impl State {
    pub fn new(start_timestamp: i64, time_spent_start: i64) -> Self {
        Self {
            start_timestamp,
            time_spent_start,
            windows: WindowsState::default(),
        }
    }
}

pub struct WindowsState {
    pub app_info: AppInfoWindowState,
    pub game_settings: GameSettingsWindowState,
    pub graphics_settings: GraphicsSettingsWindowState,
    pub stats: StatsWindowState,
    pub game_question: QuestionWindowState,
}

impl Default for WindowsState {
    fn default() -> Self {
        Self {
            app_info: AppInfoWindowState::default(),
            game_settings: GameSettingsWindowState::default(),
            graphics_settings: GraphicsSettingsWindowState::default(),
            stats: StatsWindowState::default(),
            game_question: QuestionWindowState::default(),
        }
    }
}

pub struct AppInfoWindowState {
    pub opened: bool,
}

impl Default for AppInfoWindowState {
    fn default() -> Self {
        Self { opened: false }
    }
}

pub struct GameSettingsWindowState {
    pub opened: bool,
    pub constellation_setting_learning_stage: enums::GameLearningStage,
}

pub struct QuestionWindowState {
    pub opened: bool,
}

impl Default for GameSettingsWindowState {
    fn default() -> Self {
        Self {
            opened: false,
            constellation_setting_learning_stage: enums::GameLearningStage::None,
        }
    }
}

pub struct GraphicsSettingsWindowState {
    pub opened: bool,
}

impl Default for GraphicsSettingsWindowState {
    fn default() -> Self {
        Self { opened: false }
    }
}
impl Default for QuestionWindowState {
    fn default() -> Self {
        Self { opened: true }
    }
}

pub struct StatsWindowState {
    pub opened: bool,
}

impl Default for StatsWindowState {
    fn default() -> Self {
        Self { opened: false }
    }
}
