use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (h, m) = Self::calc_time(hours, minutes);
        
        Self { hours: h, minutes: m }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let new_mins: i32 = self.minutes + minutes;

        if new_mins < 60 && new_mins >= 0 {
            return Self { minutes: self.minutes + minutes, hours: self.hours }
        };

        let (h, m) = Self::calc_time(self.hours, new_mins);

        Self { minutes: m, hours: h }
    }

    fn calc_time(mut hour: i32, mut minutes: i32) -> (i32, i32) {
        if minutes < 0 {
            while minutes < 0 {
                minutes += 60;
                hour -= 1;
            };
        } else {
            while minutes >= 60 {
                minutes -= 60;
                hour += 1
            }
        };
        hour = hour.rem_euclid(24);
        (hour, minutes)
    }

}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut m: String = self.minutes.to_string();
        let mut h: String = self.hours.to_string();

        if self.hours < 10 {
            h = "0".to_string() + &h;
        };

        if self.minutes < 10 {
            m = "0".to_string() + &m;
        };

        write!(f, "{}:{}", h, m)
    }
}