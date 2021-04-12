use macroquad::prelude::*;

pub struct Timer {
    time: f64,
    pub delay: f64,
}

impl Timer {
    pub fn from_seconds(delay: f64) -> Self {
        Self {
            time: get_time(),
            delay,
        }
    }

    /// Updates the tracked time and returns true if the delay has just been reached.
    pub fn tick_and_finished(&mut self) -> bool {
        let current_time = get_time();
        let finished = current_time - self.time > self.delay;

        if finished {
            self.time = current_time;
        }
        finished
    }
}
