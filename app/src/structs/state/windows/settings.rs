use std::fmt::Display;

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

#[derive(PartialEq)]
pub enum GameSettingsType {
    Basic,
    Advanced,
}

impl AsRef<str> for GameSettingsType {
    fn as_ref(&self) -> &str {
        match *self {
            Self::Basic => "Basic",
            Self::Advanced => "Advanced",
        }
    }
}

pub struct GameSettingsWindowState {
    pub settings_type: GameSettingsType,
    pub subwindow: GameSettingsSubWindow,
    pub questions_subwindow: GameSettingsQuestionsSubWindowState,
    pub generated_query: String,
    pub query: String,
}

impl Default for GameSettingsWindowState {
    fn default() -> Self {
        Self {
            settings_type: GameSettingsType::Basic,
            subwindow: GameSettingsSubWindow::General,
            questions_subwindow: GameSettingsQuestionsSubWindowState::default(),
            generated_query: String::new(),
            query: String::new(),
        }
    }
}

pub struct GameSettingsQuestionsSubWindowState {
    pub subwindow: GameSettingsQuestionsSubWindow,
}

impl Default for GameSettingsQuestionsSubWindowState {
    fn default() -> Self {
        Self {
            subwindow: GameSettingsQuestionsSubWindow::FindThisObject,
        }
    }
}

pub struct SkySettingsWindowState {
    pub subwindow: SkySettingsSubWindow,

    pub groups_subwindow_state: sg_game_constellations::GameConstellationsState,
}

#[allow(clippy::derivable_impls)]
impl Default for SkySettingsWindowState {
    fn default() -> Self {
        Self {
            subwindow: SkySettingsSubWindow::General,
            groups_subwindow_state: sg_game_constellations::GameConstellationsState::default(),
        }
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
            Self::Sky => "Sky and theme settings",
        }
    }
}

impl Display for SettingsSubWindow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(PartialEq)]
pub enum GameSettingsSubWindow {
    General,
    Questions,
    Constellations,
}

impl AsRef<str> for GameSettingsSubWindow {
    fn as_ref(&self) -> &str {
        match *self {
            Self::General => "General",
            Self::Questions => "Questions",
            Self::Constellations => "Constellations",
        }
    }
}

impl Display for GameSettingsSubWindow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(PartialEq)]
pub enum GameSettingsQuestionsSubWindow {
    FindThisObject,
    WhatIsThisObject,
    WhichConstellationIsThisPointIn,
    GuessTheAngularDistance,
    GuessTheCoordinates,
    GuessTheMagnitude,
}

impl AsRef<str> for GameSettingsQuestionsSubWindow {
    fn as_ref(&self) -> &str {
        match *self {
            Self::FindThisObject => "Find this object",
            Self::WhatIsThisObject => "What is this object",
            Self::WhichConstellationIsThisPointIn => "Which constellation is this point in",
            Self::GuessTheAngularDistance => "Guess the angular distance",
            Self::GuessTheCoordinates => "Guess the coordinates",
            Self::GuessTheMagnitude => "Guess the magnitude",
        }
    }
}

impl Display for GameSettingsQuestionsSubWindow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(PartialEq)]
pub enum SkySettingsSubWindow {
    General,
    Stars,
    Deepsky,
    Lines,
    Markers,
}

impl AsRef<str> for SkySettingsSubWindow {
    fn as_ref(&self) -> &str {
        match *self {
            Self::General => "General",
            Self::Stars => "Stars",
            Self::Deepsky => "Deepsky objects",
            Self::Lines => "Lines",
            Self::Markers => "Markers",
        }
    }
}

impl Display for SkySettingsSubWindow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
