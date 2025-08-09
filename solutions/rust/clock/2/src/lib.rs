use std::fmt::Display;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Clock {
    minutes: u16,
}

impl Clock {
    const MINUTES_PER_HOUR: i32 = 60;
    const HOURS_PER_DAY: i32 = 24;
    const MINUTES_PER_DAY: i32 = Self::MINUTES_PER_HOUR * Self::HOURS_PER_DAY;

    pub fn new(hours: i32, minutes: i32) -> Self {
        // Need to mod minutes by `MINUTES_PER_DAY` to prevent possible overflow if minutes is at i32:MAX and then we add to it
        Self::default().add_minutes(
            hours % Self::HOURS_PER_DAY * Self::MINUTES_PER_HOUR + minutes % Self::MINUTES_PER_DAY,
        )
    }

    /// Increments the number of minutes in `self` by `minutes` rolling over if necessary
    pub fn add_minutes(&mut self, mut minutes: i32) -> Self {
        // Need to mod minutes by `MINUTES_PER_DAY` to prevent possible overflow if minutes is at i32:MAX and then we add to it
        minutes %= Self::MINUTES_PER_DAY;
        minutes += self.minutes as i32; // Get total minutes
        minutes %= Self::MINUTES_PER_DAY;
        if minutes < 0 {
            minutes += Self::MINUTES_PER_DAY;
        }
        self.minutes = minutes
            .try_into()
            .expect("Should be in correct range at this point");
        *self
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hours = self.minutes as i32 / Self::MINUTES_PER_HOUR;
        let minutes = self.minutes as i32 % Self::MINUTES_PER_HOUR;
        write!(f, "{hours:02}:{minutes:02}")
    }
}
