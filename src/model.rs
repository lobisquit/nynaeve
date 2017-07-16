extern crate chrono;

use self::chrono::NaiveTime;
use self::chrono::Timelike;
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl<'a> Into<i32> for &'a Weekday {
    fn into(self) -> i32 {
        match *self {
            Weekday::Monday => 0,
            Weekday::Tuesday => 1,
            Weekday::Wednesday => 2,
            Weekday::Thursday => 3,
            Weekday::Friday => 4,
            Weekday::Saturday => 5,
            Weekday::Sunday => 6,
        }
    }
}

impl Weekday {
    pub fn from_int(value: i32) -> Result<Weekday, String> {
        match value {
            0 => Ok(Weekday::Monday),
            1 => Ok(Weekday::Tuesday),
            2 => Ok(Weekday::Wednesday),
            3 => Ok(Weekday::Thursday),
            4 => Ok(Weekday::Friday),
            5 => Ok(Weekday::Saturday),
            6 => Ok(Weekday::Sunday),
            _ => Err(format!("{} is not in 0..7: invalid weekday number", value)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Place {
    name: String,
}

impl Place {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn new<S: Into<String>>(name: S) -> Place {
        let place_name: String = name.into();
        Place { name: place_name }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Time {
    hour: i32,
    minute: i32,
}

impl Time {
    pub fn hour(&self) -> i32 {
        self.hour
    }

    pub fn minute(&self) -> i32 {
        self.minute
    }

    pub fn new(hour: i32, minute: i32) -> Result<Time, String> {
        if hour <= 24 && minute <= 60 {
            Ok(Time {
                hour: hour,
                minute: minute,
            })
        } else {
            Err(format!("{}:{} is and invalid Time", hour, minute))
        }
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Time) -> Option<Ordering> {
        if self.hour == other.hour {
            Some(self.minute.cmp(&other.minute))
        } else {
            Some(self.hour.cmp(&other.hour))
        }
    }
}

// conversion from / into NaiveTime gives us ToSql trait
// I cannot find an actual implementation of such trait

impl<'a> Into<NaiveTime> for &'a Time {
    fn into(self) -> NaiveTime {
        NaiveTime::from_hms(self.hour as u32, self.minute as u32, 0)
    }
}

impl From<NaiveTime> for Time {
    fn from(t: NaiveTime) -> Self {
        Time::new(t.hour() as i32, t.minute() as i32).unwrap()
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Alarm {
    place: Place,
    weekdays: Vec<Weekday>,
    time: Time,
}

impl Alarm {
    pub fn new(place: Place, days: Vec<Weekday>, time: Time) -> Alarm {
        Alarm {
            place: place,
            weekdays: days,
            time: time,
        }
    }

    pub fn place(&self) -> &Place {
        &self.place
    }

    pub fn weekdays(&self) -> &Vec<Weekday> {
        &self.weekdays
    }

    pub fn time(&self) -> &Time {
        &self.time
    }

    pub fn subset(&self, other: &Alarm) -> bool {
        // check place and time are the same
        if other.place.eq(&self.place) && self.time.eq(&other.time) {
            // check if self.days is a subset of other.days
            for self_day in &self.weekdays {
                if !other.weekdays.contains(self_day) {
                    return false;
                }
            }
        }
        true
    }
}
