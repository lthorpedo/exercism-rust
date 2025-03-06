use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock(i32);

const MINUTES_IN_DAY: i32 = 60 * 24;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self((hours * 60 + minutes).rem_euclid(MINUTES_IN_DAY))
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(0, self.0 + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.0 / 60, self.0 % 60)
    }
}

