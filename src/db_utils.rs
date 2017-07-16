use model::*;
use model::Weekday::*;

use postgres::{Connection, TlsMode};

use std::cmp::Ordering;
use std::fmt;

extern crate chrono;
use chrono::NaiveTime;
use chrono::Timelike;

pub struct Database<'a> {
    conn: &'a Connection,
}

impl<'a> Database<'a> {
    pub fn new(conn: &'a Connection) -> Database<'a> {
        Database { conn: conn }
    }

    pub fn init(&self, reset: bool) {
        if reset {
            // one execute can have only a single query, loop needed
            let commands = [
                "DROP TABLE IF EXISTS places CASCADE;",
                "DROP TABLE IF EXISTS weekdays CASCADE;",
                "DROP TABLE IF EXISTS alarms CASCADE;",
                "DROP TABLE IF EXISTS weekdays_alarms CASCADE;",
            ];

            for command in &commands {
                self.conn
                    .execute(command, &[])
                    .ok()
                    .expect(format!("Query '{}' failed", command).as_ref());
            }
        }
        let commands = [
            "CREATE TABLE IF NOT EXISTS places (
                name VARCHAR NOT NULL,
                PRIMARY KEY (name)
            );",
            "CREATE TABLE IF NOT EXISTS weekdays (
                num INTEGER NOT NULL,
                PRIMARY KEY (num)
            );",
            "CREATE TABLE IF NOT EXISTS alarms (
                id SERIAL NOT NULL,
                time TIME NOT NULL,
                place_name VARCHAR NOT NULL,
                PRIMARY KEY (id),
                UNIQUE (time, place_name),
                FOREIGN KEY (place_name) REFERENCES places(name) ON DELETE CASCADE
            );",
            "CREATE TABLE IF NOT EXISTS weekdays_alarms (
                weekday_num INTEGER NOT NULL,
                alarm_id INTEGER NOT NULL,
                FOREIGN KEY (weekday_num) REFERENCES weekdays(num),
                FOREIGN KEY (alarm_id) REFERENCES alarms(id) ON DELETE CASCADE,
                UNIQUE (weekday_num, alarm_id)
            );",
        ];
        // create tables
        for command in &commands {
            self.conn
                .execute(command, &[])
                .ok()
                .expect(format!("Query '{}' failed", command).as_ref());
        }

        for day in 0..7 {
            self.conn
                .execute("INSERT INTO weekdays (num) VALUES ($1);", &[&day])
                .unwrap();
        }
    }

    pub fn add_place(&self, place: &Place) {
        let name: &str = place.name().as_ref();
        self.conn
            .execute("INSERT INTO places (name) VALUES ($1);", &[&name])
            .unwrap();
    }

    pub fn get_places(&self) -> Vec<Place> {
        let mut places = vec![];
        for row in &self.conn.query("SELECT name FROM places;", &[]).unwrap() {
            let name: String = row.get("name");
            places.push(Place::new(name));
        }
        places
    }

    pub fn remove_place(&self, place: Place) -> Result<(), String> {
        if !self.get_places().contains(&place) {
            Err("No such place in database".to_owned())
        } else {
            &self.conn
                .query("DELETE FROM places WHERE name = $1", &[&place.name()])
                .unwrap();
            Ok(())
        }
    }

    pub fn add_alarm(&self, alarm: &Alarm) {
        // time is converted to NaiveTime to put it easily into queries
        let time: NaiveTime = alarm.time().into();
        let place = alarm.place();
        let weekdays = alarm.weekdays();

        // add place if not in db
        if !self.get_places().contains(&place) {
            self.add_place(place);
        }

        // add alarm to its table
        self.conn
            .execute(
                "INSERT INTO alarms (time, place_name) values ($1, $2);",
                &[&time, &place.name()],
            )
            .unwrap();

        // add weekday to weekdays_alarms joint table
        for day in weekdays {
            let num: i32 = day.into();
            self.conn
                .execute(
                    "INSERT INTO weekdays_alarms
                    (SELECT $1, id FROM alarms
                        WHERE alarms.time = $2
                        AND alarms.place_name = $3);",
                    &[&num, &time, &place.name()],
                )
                .unwrap();
        }
    }

    pub fn get_alarms(&self) -> Vec<Alarm> {
        let mut alarms = vec![];

        let alarms_query = self.conn
            .query(
                "SELECT id, place_name, time
                FROM alarms, places
                WHERE alarms.place_name = places.name;", &[])
            .unwrap();
        for row in &alarms_query {
            let id: i32 = row.get("id");

            let place_name: String = row.get("place_name");
            let naive_time: NaiveTime = row.get("time");
            let time: Time = naive_time.into();

            // retrieve weekdays
            let weekdays_query = self.conn
                .query(
                    "SELECT weekday_num
                    FROM weekdays_alarms
                    WHERE weekdays_alarms.alarm_id = $1;",
                    &[&id],
                )
                .unwrap();

            let mut weekdays = vec![];
            for combination_row in &weekdays_query {
                let weekday = Weekday::from_int(
                    combination_row.get("weekday_num")
                ).unwrap();
                weekdays.push(weekday);
            }
            alarms.push(
                Alarm::new(
                    Place::new(place_name),
                    weekdays,
                    time));
        }
        alarms
    }

    pub fn remove_alarm(&self, alarm: &Alarm) -> Result<(), String> {
        if !self.get_alarms().contains(&alarm) {
            Err("No such alarm in database".to_owned())
        }
        else {
            let time: NaiveTime = alarm.time().into();
            &self.conn
                .query(
                    "DELETE FROM alarms WHERE
                    alarms.place_name = $1 AND
                    alarms.time = $2",
                    &[&alarm.place().name(), &time])
                .unwrap();
            Ok(())
        }
    }
}
