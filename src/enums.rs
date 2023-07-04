use std::fmt::Display;

pub enum Inputs {
	AltShiftI,
	AltShiftS,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum LightPollution {
	Default,
	NoSpecific,
	Prague,
}

impl Display for LightPollution {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			&Self::Default => write!(f, "Default"),
			&Self::NoSpecific => write!(f, "No specific"),
			&Self::Prague => write!(f, "Prague"),
		}
	}
}
