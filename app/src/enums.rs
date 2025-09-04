use eframe::egui;
use egui::epaint::Pos2;
use std::fmt::Display;

pub enum Inputs {
    AltShiftI,
    AltShiftO,
    AltShiftS,
    MouseMiddle,
    Space,
}

pub enum PointerPosition {
    OnScreen(Pos2),
    OffScreen,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum LightPollution {
    Default,
    NoSpecific,
    Prague,
    PragueDark,
    AverageVillage,
}

impl Display for LightPollution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Default => write!(f, "Default"),
            Self::NoSpecific => write!(f, "No specific"),
            Self::Prague => write!(f, "Prague"),
            Self::PragueDark => write!(f, "Prague (dark place)"),
            Self::AverageVillage => write!(f, "Village"),
        }
    }
}

impl LightPollution {
    pub fn explanation(&self) -> &'static str {
        match *self {
            Self::Default => "The default settings",
            Self::NoSpecific => "No specific settings, set by the user",
            Self::Prague => "Looking from a normal place in Prague",
            Self::PragueDark => "Looking from a reasonably dark (compared to the rest) place in Prague",
            Self::AverageVillage => "Looking from an average village",
        }
    }

    pub const fn variants() -> [Self; 5] {
        [Self::NoSpecific, Self::Default, Self::Prague, Self::PragueDark, Self::AverageVillage]
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum GameStage {
    Guessing,
    Checked,
    NotStartedYet,
    NoMoreQuestions,
    ScoredModeFinished,
}

pub enum StorageKeys {
    TimeSpent,
    DeepskyFilesToNotRender,
    LineFilesToNotRender,
    MarkerFilesToNotRender,
    StarFilesToNotRender,
    StarNamesFilesToNotUse,
    GameInactiveConstellations,
    GameGroupActiveConstellellations,
    GameInactiveConstellationGroups,
    GameSettings,
    GameQuestionSettings,
    SkySettings,
    Theme,
    GraphicsSettings,
    ActiveQuestionPack,
    QuestionPackQuery,
    QuestionPackDescription,
    QuestionPacks,
    InputSettings,
}

impl AsRef<str> for StorageKeys {
    fn as_ref(&self) -> &str {
        match *self {
            Self::TimeSpent => "time_spent",
            Self::DeepskyFilesToNotRender => "deepsky_files_to_not_render",
            Self::LineFilesToNotRender => "line_files_to_not_render",
            Self::MarkerFilesToNotRender => "marker_files_to_not_render",
            Self::StarFilesToNotRender => "star_files_to_not_render",
            Self::StarNamesFilesToNotUse => "star_names_files_to_not_use",
            Self::GameInactiveConstellations => "game_inactive_constellations",
            Self::GameGroupActiveConstellellations => "game_group_active_constellations",
            Self::GameInactiveConstellationGroups => "game_inactive_constellations_groups",
            Self::GameSettings => "game_settings",
            Self::GameQuestionSettings => "game_question_settings",
            Self::SkySettings => "sky_settings",
            Self::Theme => "theme",
            Self::GraphicsSettings => "graphics_settings",
            Self::ActiveQuestionPack => "active_question_pack",
            Self::QuestionPackQuery => "question_pack_query",
            Self::QuestionPackDescription => "question_pack_description",
            Self::QuestionPacks => "question_packs",
            Self::InputSettings => "input_settings",
        }
    }
}

impl Display for StorageKeys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

pub enum ScreenWidth {
    Normal,
    Narrow,
    VeryNarrow,
}

impl ScreenWidth {
    pub fn narrow(&self) -> bool {
        match *self {
            Self::Normal => false,
            Self::Narrow | Self::VeryNarrow => true,
        }
    }

    pub fn very_narrow(&self) -> bool {
        match *self {
            Self::Normal | Self::Narrow => false,
            Self::VeryNarrow => true,
        }
    }

    pub fn from_width(width: f32) -> Self {
        if width <= 600.0 {
            return Self::VeryNarrow;
        }
        if width <= 900.0 {
            return Self::Narrow;
        }
        Self::Normal
    }
}

pub enum RendererCategory {
    Stars,
    Lines,
    Deepskies,
    Markers,
}
