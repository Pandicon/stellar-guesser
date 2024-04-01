use crate::{enums, structs::graphics_settings::GraphicsSettings};

use crate::caspr::CellestialSphere;

use self::frames_handler::FramesHandler;

use crate::game::game::{self, GameHandler};

use crate::{
	input,
	structs::{frames_handler, state},
};

pub struct Application {
	pub input: input::Input,
	pub state: state::State,

	pub frame_timestamp: i64,
	pub frame_timestamp_ms: i64,
	pub cellestial_sphere: CellestialSphere,
	pub graphics_settings: GraphicsSettings,
	pub frames_handler: FramesHandler,
	pub game_handler: game::GameHandler,

	pub authors: String,
	pub version: String,
}

impl Application {
	pub fn new(ctx: &egui::Context, authors: String, version: String, storage: &mut Option<crate::storage::Storage>) -> Self {
		egui_extras::install_image_loaders(&ctx);
		ctx.set_visuals(egui::Visuals::dark());
		let mut time_spent_start = 0;
		if let Some(storage) = storage {
			if let Some(time_spent_restore) = storage.get_string("time_spent") {
				if let Ok(time_spent) = time_spent_restore.parse() {
					time_spent_start = time_spent;
				}
			}
		}
		let timestamp = chrono::Utc::now().timestamp();
		let state = state::State::new(timestamp, time_spent_start);
		let mut cellestial_sphere = CellestialSphere::load(storage).unwrap();
		cellestial_sphere.init();
		Self {
			input: input::Input::default(),
			state,

			frame_timestamp: timestamp,
			frame_timestamp_ms: chrono::Utc::now().timestamp_millis(),
			game_handler: GameHandler::init(&mut cellestial_sphere, storage),
			cellestial_sphere,
			graphics_settings: GraphicsSettings::default(),
			frames_handler: FramesHandler::default(),

			authors,
			version,
		}
	}

	pub fn update(&mut self, ctx: &egui::Context) {
		#[cfg(any(target_os = "ios", target_os = "android"))]
		ctx.input_mut(|i| i.events.push(egui::Event::Text(self.input.text_from_keys.clone())));
		self.input.input_field_had_focus_last_frame = self.input.input_field_has_focus;
		self.input.input_field_has_focus = false;
		self.frames_handler.current_frame.timestamp_ns = chrono::Local::now().timestamp_nanos();
		self.frame_timestamp = chrono::Utc::now().timestamp();
		let cursor_within_central_panel = self.render(ctx);
		self.handle_input(cursor_within_central_panel, ctx);
		self.frames_handler.handle();
		self.frames_handler.last_frame = chrono::Local::now().timestamp_nanos();
		ctx.request_repaint();
	}

	fn save(&mut self, storage: &mut crate::storage::Storage) {
		storage.set_string("time_spent", (self.state.time_spent_start + (self.frame_timestamp - self.state.start_timestamp)).to_string());

		let mut deepsky_files_to_not_render = Vec::new();
		for (file, active) in &self.cellestial_sphere.deepskies_categories_active {
			if !*active {
				deepsky_files_to_not_render.push(file.clone());
			}
		}
		storage.set_string("deepsky_files_to_not_render", deepsky_files_to_not_render.join("|"));

		let mut line_files_to_not_render = Vec::new();
		for (file, active) in &self.cellestial_sphere.lines_categories_active {
			if !*active {
				line_files_to_not_render.push(file.clone());
			}
		}
		storage.set_string("line_files_to_not_render", line_files_to_not_render.join("|"));

		let mut marker_files_to_not_render = Vec::new();
		for (file, active) in &self.cellestial_sphere.markers_categories_active {
			if !*active {
				marker_files_to_not_render.push(file.clone());
			}
		}
		storage.set_string("marker_files_to_not_render", marker_files_to_not_render.join("|"));

		let mut star_files_to_not_render = Vec::new();
		for (file, active) in &self.cellestial_sphere.stars_categories_active {
			if !*active {
				star_files_to_not_render.push(file.clone());
			}
		}
		storage.set_string("star_files_to_not_render", star_files_to_not_render.join("|"));

		let mut inactive_constellations = Vec::new();
		for (abbreviation, value) in &self.game_handler.active_constellations {
			if !*value {
				inactive_constellations.push(abbreviation.as_str());
			}
		}
		storage.set_string("game_inactive_constellations", inactive_constellations.join("|"));

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
				storage.set_string(&format!("game_group_active_constellations_{}", group), group_active_constellations.join("|"));
			}
		}

		let mut inactive_constellations_groups = Vec::new();
		for (group, value) in &self.game_handler.active_constellations_groups {
			if !value {
				inactive_constellations_groups.push(group.to_string());
			}
		}
		storage.set_string("inactive_constellations_groups", inactive_constellations_groups.join("|"));
	}
}
