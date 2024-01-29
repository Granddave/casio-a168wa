use serde::{Deserialize, Serialize};

use self::{
    datetime::{DateTime, HourFormat},
    stopwatch::Stopwatch,
    time_setting::TimeSetter,
};

pub mod datetime;
pub mod stopwatch;
pub mod time_setting;

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
    #[serde(skip)]
    pub time_setting: TimeSetter,
    #[serde(skip)]
    pub mode: Mode,
    #[serde(skip)]
    pub illuminator: bool,
    #[serde(skip)]
    illuminator_timeout: u64,
    pub hour_format: HourFormat,

    #[serde(skip)]
    pub stopwatch: Stopwatch,
}

impl Clock {
    pub fn tick_hundreds(&mut self) {
        if self.illuminator_timeout > 0 {
            self.illuminator_timeout -= 1;
            if self.illuminator_timeout == 0 {
                self.illuminator = false;
            }
        }

        self.stopwatch.tick_hundreds();
    }

    pub fn illuminate(&mut self) {
        self.illuminator = true;
        self.illuminator_timeout = 20;
    }

    pub fn cycle_mode(&mut self) {
        self.mode.next();
    }
}
