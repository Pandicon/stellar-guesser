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
			windows: WindowsState::default()
		}
	}
}

pub struct WindowsState {
	pub app_info: AppInfoWindowState,
	pub stats: StatsWindowState
}

impl Default for WindowsState {
	fn default() -> Self {
		Self {
			app_info: AppInfoWindowState::default(),
			stats: StatsWindowState::default()
		}
	}
}

pub struct AppInfoWindowState {
	pub opened: bool,
}

impl Default for AppInfoWindowState {
	fn default() -> Self {
		Self {
			opened: false
		}
	}
}

pub struct StatsWindowState {
	pub opened: bool,
}

impl Default for StatsWindowState {
	fn default() -> Self {
		Self {
			opened: false
		}
	}
}