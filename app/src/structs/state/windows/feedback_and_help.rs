pub struct FeedbackAndHelpWindowState {
    pub opened: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for FeedbackAndHelpWindowState {
    fn default() -> Self {
        Self { opened: false }
    }
}
