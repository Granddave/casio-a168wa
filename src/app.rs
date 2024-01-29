use serde::{Deserialize, Serialize};

use self::clock::{Clock, Mode};

pub mod clock;

/// Module that containg functions that save and restore clock state from disk
mod persistence;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct App {
    pub should_quit: bool,
    tick_counter: u64,
    tick_rate_ms: u64,
    pub clock: Clock,
}

impl App {
    pub fn new(tick_rate_ms: u64) -> Self {
        Self {
            tick_rate_ms,
            clock: persistence::restore().unwrap_or_default(),
            ..Default::default()
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        self.tick_counter += 1;
        if self.tick_counter % (1000 / self.tick_rate_ms) == 0 {
            self.tick_counter = 0;
            self.clock.datetime.increment_second(true);
        }

        if self.tick_counter % (10 / self.tick_rate_ms) == 0 {
            self.clock.tick_hundreds();
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
        self.clock.illuminate();
        match self.clock.mode {
            Mode::TimeSetting => self.clock.time_setting.next_field(),
            Mode::Stopwatch => match self.clock.stopwatch.started {
                true => self.clock.stopwatch.split(),
                false => match self.clock.stopwatch.split {
                    Some(_) => self.clock.stopwatch.split(),
                    None => self.clock.stopwatch.clear(),
                },
            },
            _ => {}
        }
    }

    pub fn press_button_b(&mut self) {
        self.clock.mode.next();
    }

    pub fn press_button_c(&mut self) {
        match self.clock.mode {
            Mode::Timekeeping => self.clock.hour_format.next(),
            Mode::Alarm => {}
            Mode::Stopwatch => match self.clock.stopwatch.started {
                true => self.clock.stopwatch.stop(),
                false => self.clock.stopwatch.start(),
            },
            Mode::TimeSetting => self.clock.time_setting.activate(&mut self.clock.datetime),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::app::clock::stopwatch::Stopwatch;

    use super::*;

    fn sleep_ms(app: &mut App, ms: u64) {
        for _ in 0..ms {
            app.tick();
        }
    }

    #[test]
    fn test_stopwatch_start_stop() {
        let mut app = App::new(1);

        // Go to stop watch mode
        app.press_button_b();
        app.press_button_b();

        for _ in 0..4 {
            // Start the stop watch
            app.press_button_c();
            sleep_ms(&mut app, 1000);
            // Stop the stop watch
            app.press_button_c();
            sleep_ms(&mut app, 1000);
        }

        assert_eq!(app.clock.stopwatch.measurement.hundreds, 0);
        assert_eq!(app.clock.stopwatch.measurement.seconds, 4);
        assert_eq!(app.clock.stopwatch.measurement.minutes, 0);
        assert_eq!(app.clock.stopwatch.started, false);

        // Clear the stop watch
        app.press_button_a();
        assert_eq!(app.clock.stopwatch, Stopwatch::default());
    }

    #[test]
    fn test_stopwatch_split_measurement() {
        let mut app = App::new(1);

        // Go to stop watch mode
        app.press_button_b();
        app.press_button_b();

        // Start the stop watch
        app.press_button_c();
        assert_eq!(app.clock.stopwatch.started, true);
        sleep_ms(&mut app, 1000);

        // Split
        assert!(app.clock.stopwatch.split.is_none());
        app.press_button_a();
        sleep_ms(&mut app, 1000);
        assert!(app.clock.stopwatch.split.is_some());

        // Split release
        app.press_button_a();
        sleep_ms(&mut app, 1000);
        assert!(app.clock.stopwatch.split.is_none());

        // stop the stop watch
        app.press_button_c();
        assert_eq!(app.clock.stopwatch.started, false);
        sleep_ms(&mut app, 1000);

        assert_eq!(app.clock.stopwatch.measurement.hundreds, 0);
        assert_eq!(app.clock.stopwatch.measurement.seconds, 3);
        assert_eq!(app.clock.stopwatch.measurement.minutes, 0);
        assert_eq!(app.clock.stopwatch.started, false);

        // Clear the stop watch
        app.press_button_a();
        assert_eq!(app.clock.stopwatch, Stopwatch::default());
    }

    #[test]
    fn test_stopwatch_split_time_place_times() {
        let mut app = App::new(1);

        // Go to stop watch mode
        app.press_button_b();
        app.press_button_b();

        // Start the stop watch
        app.press_button_c();
        assert_eq!(app.clock.stopwatch.started, true);
        sleep_ms(&mut app, 1000);

        // Split. First runner finishes.
        assert!(app.clock.stopwatch.split.is_none());
        app.press_button_a();
        assert!(app.clock.stopwatch.split.is_some());
        sleep_ms(&mut app, 1000);

        // Start the stop watch
        app.press_button_c();
        sleep_ms(&mut app, 1000);

        // Split. Second runner finishes. Record time first runner.
        assert!(app.clock.stopwatch.split.is_some());
        app.press_button_a();
        assert!(app.clock.stopwatch.split.is_none());

        assert_eq!(app.clock.stopwatch.measurement.hundreds, 0);
        assert_eq!(app.clock.stopwatch.measurement.seconds, 2);
        assert_eq!(app.clock.stopwatch.measurement.minutes, 0);
        assert_eq!(app.clock.stopwatch.started, false);

        // Clear the stop watch
        app.press_button_a();
        assert_eq!(app.clock.stopwatch, Stopwatch::default());
    }
}
