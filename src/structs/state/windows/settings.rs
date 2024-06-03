use std::fmt::Display;

use crate::enums;

pub struct SettingsWindowState {
    pub opened: bool,
    pub subwindow: SettingsSubWindow,
    pub game_settings: GameSettingsWindowState,
    pub sky_settings: SkySettingsWindowState,
}

impl Default for SettingsWindowState {
    fn default() -> Self {
        Self {
            opened: false,
            subwindow: SettingsSubWindow::Game,
            game_settings: GameSettingsWindowState::default(),
            sky_settings: SkySettingsWindowState::default(),
        }
    }
}

pub struct GameSettingsWindowState {
    pub constellation_setting_learning_stage: enums::GameLearningStage,
}

impl Default for GameSettingsWindowState {
    fn default() -> Self {
        Self {
            constellation_setting_learning_stage: enums::GameLearningStage::None,
        }
    }
}

pub struct SkySettingsWindowState {}

#[allow(clippy::derivable_impls)]
impl Default for SkySettingsWindowState {
    fn default() -> Self {
        Self {}
    }
}

#[derive(PartialEq)]
pub enum SettingsSubWindow {
    Game,
    Sky,
}

impl AsRef<str> for SettingsSubWindow {
    fn as_ref(&self) -> &str {
        match *self {
            Self::Game => "Game settings",
            Self::Sky => "Sky settings",
        }
    }
}

impl Display for SettingsSubWindow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
