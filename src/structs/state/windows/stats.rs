pub struct StatsWindowState {
    pub opened: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for StatsWindowState {
    fn default() -> Self {
        Self { opened: false }
    }
}
