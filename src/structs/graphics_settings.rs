pub struct GraphicsSettings {
    pub use_default_star_colour: bool,
}

impl Default for GraphicsSettings {
    fn default() -> Self {
        Self { use_default_star_colour: false }
    }
}
