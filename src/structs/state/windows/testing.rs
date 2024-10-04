pub struct TestingWindowState {
    pub opened: bool,
}

impl Default for TestingWindowState {
    fn default() -> Self {
        Self { opened: false }
    }
}
