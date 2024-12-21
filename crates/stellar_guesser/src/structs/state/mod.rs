pub mod threads_communication;
pub mod windows;

pub struct State {
    pub start_timestamp: i64,
    pub time_spent_start: i64,

    pub windows: windows::WindowsState,
}

impl State {
    pub fn new(start_timestamp: i64, time_spent_start: i64) -> Self {
        Self {
            start_timestamp,
            time_spent_start,
            windows: windows::WindowsState::default(),
        }
    }
}
