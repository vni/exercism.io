#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: u8,
    minutes: u8,
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    fn normalize(mut h: i32, mut m: i32) -> Self {
        while m < 0 {
            m += 60;
            h -= 1;
        }
        while m >= 60 {
            m -= 60;
            h += 1;
        }

        while h < 0 {
            h += 24;
        }
        while h >= 24 {
            h -= 24;
        }

        Clock {
            hours: h as u8,
            minutes: m as u8,
        }
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock::normalize(hours, minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::normalize(self.hours as i32, self.minutes as i32 + minutes)
    }
}
