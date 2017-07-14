use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday
}

impl<'a> Into<usize> for &'a Weekday {
    fn into(self) -> usize {
        match *self {
            Weekday::Monday    => 0,
            Weekday::Tuesday   => 1,
            Weekday::Wednesday => 2,
            Weekday::Thursday  => 3,
            Weekday::Friday    => 4,
            Weekday::Saturday  => 5,
            Weekday::Sunday    => 6
        }
    }
}

impl Weekday {
    pub fn from_int(value: usize) -> Result<Weekday, String> {
        match value {
            0 => Ok(Weekday::Monday),
            1 => Ok(Weekday::Tuesday),
            2 => Ok(Weekday::Wednesday),
            3 => Ok(Weekday::Thursday),
            4 => Ok(Weekday::Friday),
            5 => Ok(Weekday::Saturday),
            6 => Ok(Weekday::Sunday),
            _ => Err(format!("{} is not in 0..7: invalid weekday number", value))
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Place {
    pub name: String,
}

impl Place {
    pub fn new<S: Into<String>>(name: S) -> Place {
        let place_name: String = name.into();
        Place{ name: place_name }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Time {
    pub hour: usize,
    pub minute: usize,
}

impl Time {
    pub fn new(hour: usize, minute: usize) -> Result<Time, String> {
        if hour <= 24 && minute <= 60 {
            Ok(Time {hour: hour, minute: minute})
        }
        else {
            Err(format!("{}:{} is and invalid Time", hour, minute))
        }
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.hour, self.minute)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Alarm {
    pub place: Place,
    pub days: Vec<Weekday>,
    pub time: Time,
}

impl Alarm {
    pub fn subset(&self, other: &Alarm) -> bool {
        // check place and time are the same
        if other.place.eq(&self.place) && self.time.eq(&other.time) {
            // check if self.days is a subset of other.days
            for self_day in &self.days {
                if ! other.days.contains(self_day) {
                    return false;
                }
            }
        }
        true
    }
}
