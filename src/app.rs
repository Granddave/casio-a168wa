use self::clock::DateTime;

pub mod clock;

#[derive(Debug, Default)]
pub struct App {
    pub should_quit: bool,
    pub tick_counter: u64,
    tick_rate_ms: u64,
    pub datetime: DateTime,
}

impl App {
    pub fn new(tick_rate_ms: u64) -> Self {
        Self {
            tick_rate_ms,
            ..Default::default()
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        self.tick_counter += 1;
        if self.tick_counter % (self.tick_rate_ms * 1000) == 0 {
            self.datetime.increment_second(true);
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn increment_counter(&mut self) {
        self.datetime.increment_second(true);
    }

    pub fn decrement_counter(&mut self) {
        self.datetime.decrement_second(true);
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
}
