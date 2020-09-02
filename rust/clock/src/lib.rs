use std::fmt;

#[derive(PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Clock {
        let calculated = Self::calc_minutes(minutes);
        let mut adjusted_hours = hours + calculated.0;
        adjusted_hours = if adjusted_hours > 0 { adjusted_hours } else { (adjusted_hours % 24) + 24 };
        
        Clock { 
            hours: adjusted_hours % 24, 
            minutes: calculated.1 
        }
    }

    fn calc_minutes(minutes: i32) -> (i32, i32) {
        let mut minutes_quotient = minutes / 60;
        let mut minutes_remainder = minutes % 60;
        if minutes_remainder < 0 {
            minutes_quotient += -1;
            minutes_remainder += 60;
        }
        (minutes_quotient, minutes_remainder)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let calculated = Self::calc_minutes(minutes + self.minutes);
        let mut adjusted_hours = self.hours + calculated.0;
        adjusted_hours = if adjusted_hours > 0 { adjusted_hours } else { (adjusted_hours % 24) + 24 };

        Clock {
            hours: adjusted_hours % 24,
            minutes: calculated.1
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
