use super::datetime::{DateTime, DateTimeField};

#[derive(Debug, Default, Clone)]
pub struct TimeSetter {
    pub selected_field: DateTimeField,
    blink_timeout: u64,
    blink_state: bool,
}

impl TimeSetter {
    pub fn tick(&mut self) {
        if self.blink_timeout == 0 {
            self.blink_state = !self.blink_state;
            self.blink_timeout = 100;
        }

        self.blink_timeout -= 1;
    }

    pub fn next_field(&mut self) {
        self.selected_field.next();
    }

    pub fn activate(&mut self, datetime: &mut DateTime) {
        match self.selected_field {
            DateTimeField::Second => {
                if datetime.second > 30 {
                    datetime.increment_minute(false);
                }
                datetime.second = 0;
            }
            DateTimeField::Minute => datetime.increment_minute(false),
            DateTimeField::Hour => datetime.increment_hour(false),
            DateTimeField::Date => datetime.increment_date(false),
            DateTimeField::Month => datetime.increment_month(),
            DateTimeField::DayOfWeek => datetime.increment_day_of_week(),
        }
    }
}
