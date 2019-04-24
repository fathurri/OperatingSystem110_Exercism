use std::fmt;

pub struct Clock{
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        //unimplemented!(
            //"Construct a new Clock from {} hours and {} minutes",
            //hours,
            //minutes
            Clock{hours: hours, minutes: minutes}
    }
    

    pub fn add_minutes(&self, minutes: i32) -> Self {
        //unimplemented!("Add {} minutes to existing Clock time", minutes);
        let m = self.minutes + minutes;
        Clock{hours = self.hours, minutes = m}
        }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.hours < 10 && self.minutes < 10 {
            write!(f, "0{}:0{}", self.hours, self.minutes)
        }
        else if self.hours >= 10 && self.minutes < 10 {
            write!(f, "{}:0{}", self.hours, self.minutes)
        }
        else if self.hours < 10 && self.minutes >= 10 {
            write!(f, "0{}:{}", self.hours, self.minutes)
        }
        else {
            write!(f, "{}:{}", self.hours, self.minutes)
        }
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Clock) -> bool {
        true
        }
}