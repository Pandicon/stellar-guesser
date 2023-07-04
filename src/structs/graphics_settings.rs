use eframe::epaint::Color32;

pub struct GraphicsSettings {
	pub default_star_colour: Color32,
	pub use_default_star_colour: bool,
}

impl Default for GraphicsSettings {
	fn default() -> Self {
		Self {
			default_star_colour: Color32::WHITE,
			use_default_star_colour: false,
		}
	}
}
