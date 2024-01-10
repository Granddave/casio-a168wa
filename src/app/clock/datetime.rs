use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum HourFormat {
    Format12,
    #[default]
    Format24,
}

impl HourFormat {
    pub fn next(&mut self) {
        *self = match *self {
            HourFormat::Format12 => HourFormat::Format24,
            HourFormat::Format24 => HourFormat::Format12,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum Weekday {
    #[default]
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl fmt::Display for Weekday {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Weekday::Monday => write!(f, "MO"),
            Weekday::Tuesday => write!(f, "TU"),
            Weekday::Wednesday => write!(f, "WE"),
            Weekday::Thursday => write!(f, "TH"),
            Weekday::Friday => write!(f, "FR"),
            Weekday::Saturday => write!(f, "SA"),
            Weekday::Sunday => write!(f, "SU"),
        }
    }
}

impl Weekday {
    pub fn increment(&mut self) {
        match self {
            Weekday::Monday => *self = Weekday::Tuesday,
            Weekday::Tuesday => *self = Weekday::Wednesday,
            Weekday::Wednesday => *self = Weekday::Thursday,
            Weekday::Thursday => *self = Weekday::Friday,
            Weekday::Friday => *self = Weekday::Saturday,
            Weekday::Saturday => *self = Weekday::Sunday,
            Weekday::Sunday => *self = Weekday::Monday,
        }
    }

    pub fn decrement(&mut self) {
        match self {
            Weekday::Monday => *self = Weekday::Sunday,
            Weekday::Tuesday => *self = Weekday::Monday,
            Weekday::Wednesday => *self = Weekday::Tuesday,
            Weekday::Thursday => *self = Weekday::Wednesday,
            Weekday::Friday => *self = Weekday::Thursday,
            Weekday::Saturday => *self = Weekday::Friday,
            Weekday::Sunday => *self = Weekday::Saturday,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateTime {
    /// 0-23 hour
    pub hour: u8,
    /// 0-59 minute
    pub minute: u8,
    /// 0-59 second
    pub second: u8,
    /// Months 1-12
    pub month: u8,
    /// Days 1-31
    pub date: u8,
    pub day_of_week: Weekday,
}

impl Default for DateTime {
    fn default() -> Self {
        Self {
            hour: 00,
            minute: 0,
            second: 0,
            month: 1,
            date: 1,
            day_of_week: Weekday::Monday,
        }
    }
}

impl DateTime {
    fn days_per_month(month: u8) -> u8 {
        match month {
            1 => 31,
            2 => 28, // Leap year is not considered
            3 => 31,
            4 => 30,
            5 => 31,
            6 => 30,
            7 => 31,
            8 => 30,
            9 => 31,
            10 => 30,
            11 => 31,
            12 => 30,
            _ => panic!("Month out of range: {}", month),
        }
    }

    pub fn increment_hour(&mut self, increment_date: bool) {
        self.hour += 1;
        self.hour %= 24;
        if self.hour == 0 && increment_date {
            self.increment_date(increment_date);
        }
    }

    pub fn decrement_hour(&mut self, decrement_date: bool) {
        if let Some(res) = self.hour.checked_sub(1) {
            self.hour = res;
        } else {
            self.hour = 23;
            if decrement_date {
                self.decrement_date(decrement_date);
            }
        }
    }

    pub fn increment_minute(&mut self, increment_hour: bool) {
        self.minute += 1;
        self.minute %= 60;
        if self.minute == 0 && increment_hour {
            self.increment_hour(increment_hour);
        }
    }

    pub fn decrement_minute(&mut self, decrement_hour: bool) {
        if let Some(res) = self.minute.checked_sub(1) {
            self.minute = res;
        } else {
            self.minute = 59;
            if decrement_hour {
                self.decrement_hour(decrement_hour);
            }
        }
    }

    pub fn increment_second(&mut self, increment_minute: bool) {
        self.second += 1;
        self.second %= 60;
        if self.second == 0 && increment_minute {
            self.increment_minute(increment_minute);
        }
    }

    pub fn decrement_second(&mut self, decrement_minute: bool) {
        if let Some(res) = self.second.checked_sub(1) {
            self.second = res;
        } else {
            self.second = 59;
            if decrement_minute {
                self.decrement_minute(decrement_minute);
            }
        }
    }

    pub fn increment_date(&mut self, increment_others: bool) {
        self.date += 1;
        if self.date > Self::days_per_month(self.month) {
            self.date = 1;
            if increment_others {
                self.increment_month();
            }
        }
        if increment_others {
            self.day_of_week.increment();
        }
    }

    pub fn decrement_date(&mut self, decrement_others: bool) {
        self.date -= 1;
        if self.date == 0 {
            self.date = Self::days_per_month(self.month);
            if decrement_others {
                self.decrement_month();
            }
        }
        if decrement_others {
            self.day_of_week.decrement();
        }
    }

    pub fn increment_month(&mut self) {
        self.month += 1;
        self.month %= 12;
    }

    pub fn decrement_month(&mut self) {
        if self.month > 1 {
            self.month -= 1;
        } else {
            self.month = 12;
        }
    }
}
