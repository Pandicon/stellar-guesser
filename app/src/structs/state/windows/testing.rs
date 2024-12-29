pub struct TestingWindowState {
    pub opened: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for TestingWindowState {
    fn default() -> Self {
        Self { opened: false }
    }
}
