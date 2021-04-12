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

pub struct Cooldown {
    /// Time in which the cooldown will be available.
    available_time: f64,
    /// The duration of the cooldown.
    duration: f64,
}

impl Cooldown {
    /// Creates a cooldown that is available at start.
    pub const fn from_seconds(duration: f64) -> Self {
        Self {
            duration,
            available_time: 0.0,
        }
    }

    /// Starts the cooldown, making it unavailable for the given duration.
    pub fn start(&mut self) {
        self.available_time = get_time() + self.duration;
    }

    /// Resets the cooldown, making it available again.
    pub fn reset(&mut self) {
        self.available_time = 0.0;
    }

    /// Returns true if the cooldown is available.
    pub fn available(&self) -> bool {
        self.available_time <= get_time()
    }
}
