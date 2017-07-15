use std::fmt;
use std::cmp::Ordering;

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
    name: String,
}

impl Place {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn new<S: Into<String>>(name: S) -> Place {
        let place_name: String = name.into();
        Place{ name: place_name }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Time {
    hour: usize,
    minute: usize,
}

impl Time {
    pub fn hour(&self) -> usize {
        self.hour
    }

    pub fn minute(&self) -> usize {
        self.minute
    }

    pub fn new(hour: usize, minute: usize) -> Result<Time, String> {
        if hour <= 24 && minute <= 60 {
            Ok(Time {hour: hour, minute: minute})
        }
        else {
            Err(format!("{}:{} is and invalid Time", hour, minute))
        }
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Time) -> Option<Ordering> {
        if self.hour == other.hour {
            Some(self.minute.cmp(&other.minute))
        }
        else {
            Some(self.hour.cmp(&other.hour))
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
    place: Place,
    days: Vec<Weekday>,
    time: Time,
}

impl Alarm {
    pub fn new(place: Place, days: Vec<Weekday>, time: Time) -> Alarm {
        Alarm { place: place, days: days, time: time }
    }

    pub fn place(&self) -> &Place {
        &self.place
    }

    pub fn days(&self) -> &Vec<Weekday> {
        &self.days
    }

    pub fn time(&self) -> &Time {
        &self.time
    }

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
