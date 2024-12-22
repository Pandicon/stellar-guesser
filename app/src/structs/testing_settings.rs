#[derive(serde::Deserialize, serde::Serialize)]
pub struct TestingSettings {
    pub highlight_stars_in_constellation: String,
    pub highlight_stars_in_constellation_precomputed: String,
}

#[allow(clippy::derivable_impls)]
impl Default for TestingSettings {
    fn default() -> Self {
        Self {
            highlight_stars_in_constellation: String::new(),
            highlight_stars_in_constellation_precomputed: String::new(),
        }
    }
}
