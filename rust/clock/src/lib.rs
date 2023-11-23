use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Clock {
    hours: u8,
    minutes: u8,
}

impl Clock {
    pub fn new(mut hours: i32, minutes: i32) -> Self {
        hours %= 24;
        if hours < 0 {
            hours += 24;
        }
        let mut result = Self {
            hours: hours.try_into().expect("Should be in correct range"),
            minutes: 0,
        };
        result.add_minutes(minutes)
    }

    pub fn add_minutes(&mut self, mut minutes: i32) -> Self {
        minutes += self.minutes as i32; // Get total minutes
        let mut hours = minutes / 60 + self.hours as i32; // Extract number of hours
        minutes %= 60; // Get remainder of minutes
        if minutes < 0 {
            // Ensure minutes is positive
            minutes += 60;
            hours -= 1;
        }
        hours %= 24;
        if hours < 0 {
            // Ensure hours is positive
            hours += 24;
        }
        self.hours = hours.try_into().expect("Should be in correct range");
        self.minutes = minutes.try_into().expect("Should be in correct range");
        *self
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
