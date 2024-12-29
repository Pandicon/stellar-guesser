pub struct CreditsWindowState {
    pub opened: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for CreditsWindowState {
    fn default() -> Self {
        Self { opened: false }
    }
}
