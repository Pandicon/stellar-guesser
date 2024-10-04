#[derive(serde::Deserialize, serde::Serialize)]
pub struct TestingSettings {
    pub highlight_stars_in_constellation: String,
}

#[allow(clippy::derivable_impls)]
impl Default for TestingSettings {
    fn default() -> Self {
        Self {
            highlight_stars_in_constellation: String::new(),
        }
    }
}
