pub struct AppInfoWindowState {
    pub opened: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for AppInfoWindowState {
    fn default() -> Self {
        Self { opened: false }
    }
}
