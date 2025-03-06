
#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self { Duration(s) }
}

pub trait Planet {
    const SECONDS_AROUND_SUN: u64 = 31557600; // default earth seconds
    
    fn years_during(d: &Duration) -> f64 {
        (d.0 as f64) / (Self::SECONDS_AROUND_SUN as f64)
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
    const SECONDS_AROUND_SUN: u64 = 7600544;
}
impl Planet for Venus {
    const SECONDS_AROUND_SUN: u64 = 19414149;
}
impl Planet for Earth {}
impl Planet for Mars {
    const SECONDS_AROUND_SUN: u64 = 59354033;
}
impl Planet for Jupiter {
    const SECONDS_AROUND_SUN: u64 = 374355659;
}
impl Planet for Saturn {
    const SECONDS_AROUND_SUN: u64 = 929292363;
}
impl Planet for Uranus {
    const SECONDS_AROUND_SUN: u64 = 2651370019;
}
impl Planet for Neptune {
    const SECONDS_AROUND_SUN: u64 = 5200418560;
}
