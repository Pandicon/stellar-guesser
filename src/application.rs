use eframe::egui;

use crate::enums;

#[path = "./rendering/caspr.rs"]
mod caspr;

use caspr::{CellestialSphere};

#[path = "./input.rs"]
mod input;
#[path = "./structs/state.rs"]
mod state;




pub struct Application {
	pub input: input::Input,
	pub state: state::State,

	pub frame_timestamp: i64,
	pub frame_timestamp_ms: i64,
	pub cellestial_sphere: CellestialSphere,

	pub authors: String,
	pub version: String
}

impl Application {
	pub fn new(cc: &eframe::CreationContext<'_>, authors: String, version: String) -> Self {
		cc.egui_ctx.set_visuals(egui::Visuals::dark());
		let mut time_spent_start = 0;
		if let Some(storage) = cc.storage {
			if let Some(time_spent_restore) = storage.get_string("time_spent") {
				if let Ok(time_spent) = time_spent_restore.parse() {
					time_spent_start = time_spent;
				}
			}
		}
		let timestamp = chrono::Utc::now().timestamp();
		let state = state::State::new(timestamp, time_spent_start);
		Self {
			input: input::Input::default(),
			state,

			frame_timestamp: timestamp,
			frame_timestamp_ms: chrono::Utc::now().timestamp_millis(),
			cellestial_sphere: CellestialSphere::load().expect("No catalogs are present"),

			authors,
			version,
		}
	}
}

impl eframe::App for Application {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		self.frame_timestamp = chrono::Utc::now().timestamp();
		let cursor_within_central_panel = self.render(ctx);
		self.input.handle(cursor_within_central_panel, ctx);
		for input_to_handle in &self.input.to_handle {
			match input_to_handle {
				enums::Inputs::AltShiftI => self.state.windows.app_info.opened = !self.state.windows.app_info.opened,
				enums::Inputs::AltShiftS => self.state.windows.stats.opened = !self.state.windows.stats.opened,
			}
		}
		self.cellestial_sphere.zoom(self.input.zoom / 500.0);
		ctx.request_repaint();
	}

	fn save(&mut self, storage: &mut dyn eframe::Storage) {
		storage.set_string("time_spent", (self.state.time_spent_start + (self.frame_timestamp - self.state.start_timestamp)).to_string());
	}
}