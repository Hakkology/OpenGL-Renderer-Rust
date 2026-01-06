pub struct Time {
    pub delta_time: f32,
    pub elapsed_time: f32,
    last_frame: f64,
    pub is_paused: bool,
}

impl Time {
    pub fn new() -> Self {
        Self {
            delta_time: 0.0,
            elapsed_time: 0.0,
            last_frame: 0.0,
            is_paused: false,
        }
    }

    pub fn update(&mut self, current_time: f64) {
        if self.last_frame == 0.0 {
            self.last_frame = current_time;
        }

        let actual_delta = (current_time - self.last_frame) as f32;
        self.last_frame = current_time;

        if self.is_paused {
            self.delta_time = 0.0;
        } else {
            self.delta_time = actual_delta;
            self.elapsed_time += self.delta_time;
        }
    }

    pub fn toggle_pause(&mut self) {
        self.is_paused = !self.is_paused;
    }

    /// Returns the total elapsed time in seconds since the start of the game (excluding paused time).
    pub fn time(&self) -> f32 {
        self.elapsed_time
    }
}
