pub mod app_info;
pub mod question;
pub mod settings;
pub mod stats;

pub struct WindowsState {
    pub app_info: app_info::AppInfoWindowState,
    pub settings: settings::SettingsWindowState,
    pub stats: stats::StatsWindowState,
    pub game_question: question::QuestionWindowState,
}

#[allow(clippy::derivable_impls)]
impl Default for WindowsState {
    fn default() -> Self {
        Self {
            app_info: app_info::AppInfoWindowState::default(),
            settings: settings::SettingsWindowState::default(),
            stats: stats::StatsWindowState::default(),
            game_question: question::QuestionWindowState::default(),
        }
    }
}
