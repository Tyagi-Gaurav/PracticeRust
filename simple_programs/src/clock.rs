use std::fmt;

#[derive(Debug)] //Used for printing this
#[derive(PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        return Self::normalize_clock(hours, minutes);
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        return Self::normalize_clock(self.hours, self.minutes + minutes);
    }

    fn normalize_clock(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * 60 + minutes;
        let mut effective_minutes = total_minutes % 60;
        let mut effective_hours = (total_minutes / 60) % 24;

        println!("Tm: {}, em: {}, eh: {}", total_minutes, effective_minutes, effective_hours);

        if total_minutes < 0 {
            if effective_minutes == 0 {
                effective_hours += 1;    
            } else {
                effective_minutes = 60 + effective_minutes;
            }
            effective_hours = 24 + effective_hours -1;
        }

        return Clock {
            hours: effective_hours,
            minutes: effective_minutes,
        };
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}