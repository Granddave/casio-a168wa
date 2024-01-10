use serde::{Deserialize, Serialize};

use self::datetime::{DateTime, HourFormat};

pub mod datetime;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum Mode {
    #[default]
    Timekeeping,
    Alarm,
    Stopwatch,
    TimeSetting,
}

impl Mode {
    pub fn next(&mut self) {
        *self = match *self {
            Mode::Timekeeping => Mode::Alarm,
            Mode::Alarm => Mode::Stopwatch,
            Mode::Stopwatch => Mode::TimeSetting,
            Mode::TimeSetting => Mode::Timekeeping,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Clock {
    pub datetime: DateTime,
    pub mode: Mode,
    pub illuminator: bool,
    illuminator_timeout: u64,
    pub hour_format: HourFormat,
}

impl Clock {
    pub fn tick(&mut self) {
        if self.illuminator_timeout > 0 {
            self.illuminator_timeout -= 1;
            if self.illuminator_timeout == 0 {
                self.illuminator = false;
            }
        }
    }

    pub fn illuminate(&mut self) {
        self.illuminator = true;
        self.illuminator_timeout = 20;
    }

    pub fn cycle_mode(&mut self) {
        self.mode.next();
    }
}
