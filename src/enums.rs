use egui::epaint::Pos2;
use std::fmt::Display;

pub enum Inputs {
	AltShiftG,
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
	AverageVillage,
}

impl Display for LightPollution {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match *self {
			Self::Default => write!(f, "Default"),
			Self::NoSpecific => write!(f, "No specific"),
			Self::Prague => write!(f, "Prague"),
			Self::AverageVillage => write!(f, "Village"),
		}
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum GameLearningStage {
	None,
	NotStarted,
	Learning,
	Reviewing,
	Learned,
}

impl Display for GameLearningStage {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match *self {
			Self::None => write!(f, "None"),
			Self::NotStarted => write!(f, "Not started"),
			Self::Learning => write!(f, "Learning"),
			Self::Reviewing => write!(f, "Reviewing"),
			Self::Learned => write!(f, "Learned"),
		}
	}
}

impl GameLearningStage {
	pub fn from_string(string: &str) -> Self {
		match string {
			"Learned" => Self::Learned,
			"Learning" => Self::Learning,
			"None" => Self::None,
			"Not started" => Self::NotStarted,
			"Reviewing" => Self::Reviewing,
			_ => Self::None,
		}
	}
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ColourMode {
	Dark,
	Light,
	Printing,
}

impl Display for ColourMode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match *self {
			Self::Dark => write!(f, "Dark"),
			Self::Light => write!(f, "Light"),
			Self::Printing => write!(f, "Printing"),
		}
	}
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum GameStage {
	Guessing,
	Checked,
	NotStartedYet,
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
	GameSettingsFindThisObject,
	GameSettingsWhatIsThisObject,
	GameSettingsGuessTheMagnitude,
	GameSettings,
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
			Self::GameSettingsFindThisObject => "game_settings_find_this_object",
			Self::GameSettingsWhatIsThisObject => "game_settings_what_is_this_object",
			Self::GameSettingsGuessTheMagnitude => "game_settings_guess_the_magnitude",
			Self::GameSettings => "game_settings",
		}
	}
}

impl Display for StorageKeys {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.as_ref())
	}
}
