// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { seconds: s as _ }
    }
}

pub trait Planet {
    const EARTH_DAYS_PER_YEAR: f64 = 365.25;
    const EARTH_SECONDS_PER_YEAR: f64 = Self::EARTH_DAYS_PER_YEAR * 24.0 * 60.0 * 60.0; // Days per year * Hours Per Day * Minutes Per Hour * Seconds Per Minute
    fn earth_years_ratio() -> f64;
    fn years_during(d: &Duration) -> f64 {
        d.seconds / Self::EARTH_SECONDS_PER_YEAR / Self::earth_years_ratio()
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn earth_years_ratio() -> f64 {
        0.2408467
    }
}
impl Planet for Venus {
    fn earth_years_ratio() -> f64 {
        0.61519726
    }
}
impl Planet for Earth {
    fn earth_years_ratio() -> f64 {
        1.0
    }
}
impl Planet for Mars {
    fn earth_years_ratio() -> f64 {
        1.8808158
    }
}
impl Planet for Jupiter {
    fn earth_years_ratio() -> f64 {
        11.862615
    }
}
impl Planet for Saturn {
    fn earth_years_ratio() -> f64 {
        29.447498
    }
}
impl Planet for Uranus {
    fn earth_years_ratio() -> f64 {
        84.016846
    }
}
impl Planet for Neptune {
    fn earth_years_ratio() -> f64 {
        164.79132
    }
}
