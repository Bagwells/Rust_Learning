#[derive(Debug, PartialEq)]
pub struct Clock{
   pub hours: i32,
   pub minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
       
        let total = hours * 60 + minutes;
        let norm = total.rem_euclid(1440);
        
        Self {
            hours: norm / 60,
            minutes: norm % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
}

use std::fmt;
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
