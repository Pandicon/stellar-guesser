#[derive(Clone, Copy)]
pub struct Frame {
    pub timestamp_ns: i64,
    pub duration_total_ns: i64,
    pub duration_raw_ns: i64,
}

impl Default for Frame {
    fn default() -> Self {
        Self {
            timestamp_ns: 0,
            duration_total_ns: 0,
            duration_raw_ns: 0,
        }
    }
}

pub struct FramesHandler {
    pub last_frame: i64,
    pub fps_display_holder: String,
    pub average_fps_display_holder: String,
    pub fps_current: f64,
    pub fps_average: f64,
    pub last_fps_update: i64,
    pub ms_to_wait_fps_update: i64,
    pub frames_to_hold: i64,
    pub previous_frames: Vec<Frame>,
    pub average_frame: Frame,
    pub frames_analysed: i64,
    pub current_frame: Frame,
    pub frame_index: usize,
}

impl Default for FramesHandler {
    fn default() -> Self {
        Self {
            last_frame: chrono::Local::now().timestamp_nanos_opt().expect("Date out of bounds."),
            fps_display_holder: String::new(),
            average_fps_display_holder: String::new(),
            fps_current: 0.0,
            fps_average: 0.0,
            last_fps_update: 0,
            ms_to_wait_fps_update: 250,
            frames_to_hold: 1000,
            previous_frames: Vec::new(),
            average_frame: Frame::default(),
            frames_analysed: 0,
            current_frame: Frame::default(),
            frame_index: 0,
        }
    }
}

impl FramesHandler {
    pub fn handle(&mut self) {
        let timestamp_millis = chrono::Local::now().timestamp_millis();
        self.frame_index = (self.frame_index + 1) % (self.frames_to_hold as usize);
        if self.previous_frames.len() > self.frames_to_hold as usize {
            self.frame_index = 0;
        }
        while self.previous_frames.len() > self.frames_to_hold as usize {
            self.previous_frames.remove(0);
        }
        if self.previous_frames.len() == self.frames_to_hold as usize {
            self.previous_frames[self.frame_index] = self.current_frame;
        } else {
            self.previous_frames.push(self.current_frame);
        }
        self.current_frame.duration_total_ns = chrono::Local::now().timestamp_nanos_opt().expect("Date out of bounds.") - self.last_frame;
        self.current_frame.duration_raw_ns = chrono::Local::now().timestamp_nanos_opt().expect("Date out of bounds.") - self.current_frame.timestamp_ns;
        self.fps_current = 1_000_000_000.0 / (self.current_frame.duration_total_ns as f64);
        self.average_frame = self.get_average_frame();
        self.fps_average = 1_000_000_000.0 / ((self.average_frame.duration_total_ns) as f64);
        if timestamp_millis - self.last_fps_update > self.ms_to_wait_fps_update {
            self.fps_display_holder = format!("{:.3} FPS", self.fps_current);
            self.average_fps_display_holder = format!("{:.3} aFPS", self.fps_average);
            self.last_fps_update = timestamp_millis;
        }
    }

    fn get_average_frame(&mut self) -> Frame {
        let mut total_time_ns = 0;
        let mut raw_time_ns = 0;
        for frame in &self.previous_frames {
            total_time_ns += frame.duration_total_ns;
            raw_time_ns += frame.duration_raw_ns;
        }
        self.frames_analysed = self.previous_frames.len() as i64;
        Frame {
            duration_total_ns: total_time_ns / self.frames_analysed,
            duration_raw_ns: raw_time_ns / self.frames_analysed,
            timestamp_ns: 0,
        }
    }
}
