use std::fmt;

#[derive(PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Clock {
        Self::create_clock(hours, minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::create_clock(self.hours, self.minutes + minutes)
    }

    fn create_clock(hours: i32, minutes: i32) -> Clock {
        let mut minutes_quotient = minutes / 60;
        let mut minutes_remainder = minutes % 60;
        if minutes_remainder < 0 {
            minutes_quotient += -1;
            minutes_remainder += 60;
        }

        let mut adjusted_hours = hours + minutes_quotient;
        adjusted_hours = if adjusted_hours > 0 { adjusted_hours } else { (adjusted_hours % 24) + 24 };
        
        Clock { 
            hours: adjusted_hours % 24, 
            minutes: minutes_remainder 
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
