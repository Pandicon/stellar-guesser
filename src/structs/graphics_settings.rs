use crate::enums::ColourMode;
use egui::epaint::Color32;

pub struct GraphicsSettings {
    pub default_star_colour_dark_mode: Color32,
    pub default_star_colour_light_mode: Color32,
    pub default_star_colour_print_mode: Color32,
    pub use_default_star_colour: bool,
    pub colour_mode: ColourMode,
}

impl Default for GraphicsSettings {
    fn default() -> Self {
        Self {
            default_star_colour_dark_mode: Color32::WHITE,
            default_star_colour_light_mode: Color32::BLACK,
            default_star_colour_print_mode: Color32::BLACK,
            use_default_star_colour: false,
            colour_mode: ColourMode::Dark,
        }
    }
}

impl GraphicsSettings {
    pub fn default_star_colour(&self, colour_mode: &ColourMode) -> Color32 {
        match *colour_mode {
            ColourMode::Dark => self.default_star_colour_dark_mode,
            ColourMode::Light => self.default_star_colour_light_mode,
            ColourMode::Printing => self.default_star_colour_print_mode,
        }
    }
}
