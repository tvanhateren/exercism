pub struct Duration {
    sec: u64,
}

impl From<u64> for Duration {
    fn from(sec: u64) -> Self {
        Duration { sec }
    }
}

pub trait Planet {
    const RATIO: f64;

    fn years_during(d: &Duration) -> f64 {
        Earth::years_during(d) / Self::RATIO
    }
}

pub struct Earth;

impl Planet for Earth {
    const RATIO: f64 = 31557600.;

    fn years_during(d: &Duration) -> f64 {
        d.sec as f64 / Self::RATIO
    }
}

pub struct Mercury;
impl Planet for Mercury {
    const RATIO: f64 = 0.2408467;
}

pub struct Venus;
impl Planet for Venus {
    const RATIO: f64 = 0.61519726;
}

pub struct Mars;
impl Planet for Mars {
    const RATIO: f64 = 1.8808158;
}

pub struct Jupiter;
impl Planet for Jupiter {
    const RATIO: f64 = 11.862615;
}

pub struct Saturn;
impl Planet for Saturn {
    const RATIO: f64 = 29.447498;
}

pub struct Uranus;
impl Planet for Uranus {
    const RATIO: f64 = 84.016846;
}

pub struct Neptune;
impl Planet for Neptune {
    const RATIO: f64 = 164.79132;
}
