use serde::{Deserialize, Serialize};

use self::clock::DateTime;

pub mod clock;

/// Module that containg functions that save and restore clock state from disk
mod persistence;

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Clock {
    pub datetime: DateTime,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct App {
    pub should_quit: bool,
    pub tick_counter: u64,
    tick_rate_ms: u64,
    pub clock: Clock,
}

impl App {
    pub fn new(tick_rate_ms: u64) -> Self {
        Self {
            should_quit: false,
            tick_counter: 0,
            tick_rate_ms,
            clock: persistence::restore().unwrap_or_default(),
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        self.tick_counter += 1;
        if self.tick_counter % (1000 / self.tick_rate_ms) == 0 {
            self.tick_counter = 0;
            self.clock.datetime.increment_second(true);
        }
    }

    pub fn quit(&mut self) {
        persistence::save(&self.clock).expect("Failed to save app state");
        self.should_quit = true;
    }

    pub fn increment_counter(&mut self) {
        self.clock.datetime.increment_second(true);
    }

    pub fn decrement_counter(&mut self) {
        self.clock.datetime.decrement_second(true);
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
}
