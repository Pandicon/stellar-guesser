#[derive(serde::Deserialize, serde::Serialize)]
pub struct GraphicsSettings {
    pub use_overriden_star_colour: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for GraphicsSettings {
    fn default() -> Self {
        Self { use_overriden_star_colour: false }
    }
}
