#[derive(Debug, Default, Clone, PartialEq)]
pub struct Measurement {
    pub hundreds: u64,
    pub seconds: u64,
    pub minutes: u64,
}

impl Measurement {
    pub fn tick(&mut self) {
        self.hundreds += 1;
        if self.hundreds == 100 {
            self.hundreds = 0;
            self.seconds += 1;
        }
        if self.seconds == 60 {
            self.seconds = 0;
            self.minutes += 1;
        }
        if self.minutes == 60 {
            self.minutes = 0;
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Stopwatch {
    pub measurement: Measurement,
    pub split: Option<Measurement>,
    pub started: bool,
}

impl Stopwatch {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn tick_hundreds(&mut self) {
        if !self.started {
            return;
        }

        self.measurement.tick();
    }

    pub fn clear(&mut self) {
        self.started = false;
        self.measurement = Default::default();
    }

    pub fn start(&mut self) {
        self.started = true;
    }

    pub fn stop(&mut self) {
        self.started = false;
    }

    pub fn split(&mut self) {
        match self.split {
            Some(_) => self.split = None,
            None => self.split = Some(self.measurement.clone()),
        }
    }
}
