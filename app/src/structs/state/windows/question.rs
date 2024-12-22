pub struct QuestionWindowState {
    pub opened: bool,
}

impl Default for QuestionWindowState {
    fn default() -> Self {
        Self { opened: true }
    }
}
