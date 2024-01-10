use serde::{Deserialize, Serialize};

use self::clock::DateTime;

pub mod clock;

/// Module that containg functions that save and restore clock state from disk
mod persistence;

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Clock {
    pub datetime: DateTime,
    pub illuminator: bool,
    illuminator_timeout: u64,
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

        if self.clock.illuminator_timeout > 0 {
            self.clock.illuminator_timeout -= 1;
            if self.clock.illuminator_timeout == 0 {
                self.clock.illuminator = false;
            }
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

    pub fn press_button_a(&mut self) {
        self.clock.illuminator = true;
        self.clock.illuminator_timeout = 20;
    }

    pub fn press_button_b(&mut self) {}
    pub fn press_button_c(&mut self) {}
}

#[cfg(test)]
mod tests {
    // use super::*;
}
