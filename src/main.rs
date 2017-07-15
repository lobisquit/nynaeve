extern crate postgres;
use postgres::{Connection, TlsMode};

use std::env;

mod model;
use model::*;
use model::Weekday::*;

struct Database<'a> {
    conn: &'a Connection
}

impl<'a> Database<'a> {
    fn new(conn: &'a Connection) -> Database<'a> {
        Database { conn: conn }
    }

    fn init(&self, reset: bool) {
        if reset {
            // one execute can have only a single query, loop needed
            let commands = [
                "DROP SCHEMA public CASCADE;",
                "CREATE SCHEMA public;",
                "GRANT ALL ON SCHEMA public TO postgres;",
                "GRANT ALL ON SCHEMA public TO public;",
                "COMMENT ON SCHEMA public IS 'standard public> schema';"];

            for command in &commands {
                self.conn.execute(command, &[])
                    .ok()
                    .expect(format!("Query '{}' failed", command).as_ref());
            }
        }
        let commands = [
            "CREATE TABLE if not exists place (name VARCHAR PRIMARY KEY);",
            "CREATE TABLE if not exists weekday (id INTEGER PRIMARY KEY);",
            "CREATE TABLE if not exists alarm (
                id     SERIAL PRIMARY KEY,
                hour   INTEGER NOT NULL,
                minute INTEGER NOT NULL,
                place_name VARCHAR,
                weekday_alarm INTEGER);",
            "CREATE TABLE if not exists alarm (
                id     INTEGER PRIMARY KEY,
                hour   INTEGER NOT NULL,
                minute INTEGER NOT NULL,
                place_name VARCHAR,
                weekday_alarm INTEGER);",
            "CREATE TABLE if not exists weekday_alarm (
                weekday_id INTEGER NOT NULL,
                alarm_id SERIAL NOT NULL);",
        ];
        // create tables
        for command in &commands {
            self.conn.execute(command, &[])
                .ok()
                .expect(format!("Query '{}' failed", command).as_ref());
        }

        for day in 0..7 {
            self.conn.execute("INSERT INTO weekday (id) VALUES ($1);", &[&day]);
        }
    }

    fn add_place(&self, place: &Place) {
        let name: &str = place.name().as_ref();
        self.conn.execute("INSERT INTO place (name) VALUES ($1);", &[&name]);
    }

    fn get_places(&self) -> Vec<Place> {
        let mut places = vec![];
        for row in &self.conn.query("SELECT name FROM place", &[]).unwrap() {
            let name: String = row.get(0);
            places.push(Place::new(name));
        }
        places
    }

    fn add_alarm(&self, alarm: &Alarm) {
        // add place if not in db
        if ! self.get_places().contains(&alarm.place()) {
            self.add_place(alarm.place());
        }

    }

    fn get_alarms(&self) -> Vec<Alarm> {
        let mut alarms = vec![];
        for row in &self.conn.query(
            "select place_name, hour, minute, weekday from alarm
            join place on place_name = place.name
            join weekday_alarm on alarm.id = weekday_alarm.alarm_id
            join weekday on weekday.id = weekday_alarm.weekday_id;", &[]).unwrap() {

            let name: String = row.get(0);
            places.push(Place::new(name));
        }
        places
    }
    fn get(&self) -> Vec<String> {
        let mut r = vec![];
        for row in &self.conn.query("SELECT id FROM weekday", &[]).unwrap() {
            let day_num: i32 = row.get(0);
            println!("Query => {:?}", day_num);
            r.push(Weekday::from_int(day_num as usize).unwrap());
        }
        r.iter().map(|ref day| format!("{:?}", day)).collect()
    }
}

fn main() {
    let db = env::var("DATABASE_URL").expect("No such env variable DATABASE_URL");

    let conn = Connection::connect(db, TlsMode::None).unwrap();
    let db = Database::new(&conn);
    db.init(true);
    db.add_place(&Place::new("pollo"));

    let a = Alarm::new(
        Place::new("home"),
        vec![Monday, Wednesday],
        Time::new(10, 10).unwrap());
    db.add_alarm(&a);
    println!("{:?}", db.get_places());
    // db.add_place("pollo2");
    // println!("{:?}", db.get());
    // db.lalala();

    //
    // println!("{:?}", Time::new(13, 20).unwrap() > Time::new(12, 20).unwrap());
    // println!("{:?}", Time::new(13, 20).unwrap().hour());
    // println!("{:?}",  Place::new("home").name());
    // let a = Alarm::new(
    //     Place::new("home"),
    //     vec![Monday, Wednesday],
    //     Time::new(10, 10).unwrap());
    // println!("{:?}", a);
}

// fn main() {
//     let a = Alarm {
//         place: Place::new("home"),
//         days: vec![Monday, Wednesday],
//         time: Time::new(10, 10).unwrap()
//     };
//
//     let b = Alarm {
//         place: Place::new("home"),
//         days: vec![Monday],
//         time: Time::new(10, 10).unwrap()
//     };
//     println!("{:?}", a.subset(&b));
//     println!("{:?}", b.subset(&a));
//     println!("{:?}", Time::new(12, 20).unwrap());
//     println!("{:?}", Place::new("asd".to_string()));
//
//     // for day in [Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday].into_iter() {
//     //     let number: usize =  day.into();
//     //     println!("{:?} -> {:?}", day, number);
//     // }
//     // for i in 0..10 {
//     //     match  Weekday::from_int(i) {
//     //         Ok(day)  => println!("{:?} -> {:?}", i, day),
//     //         Err(msg) => println!("{}", msg),
//     //     }
//     // }
// }
//
// fn init_db(conn: &Connection, reset: bool) {
//     if reset {
//         // erase database if reset is set
//         conn.execute("
//             DROP SCHEMA public CASCADE;
//             CREATE SCHEMA public;
//             GRANT ALL ON SCHEMA public TO postgres;
//             GRANT ALL ON SCHEMA public TO public;
//             COMMENT ON SCHEMA public IS 'standard public schema';", &[]);
//     }
//
//     // conn.execute(
//     //     format!("CREATE TABLE if not exists alarm (
//     //                 id      SERIAL      PRIMARY KEY,
//     //                 name    VARCHAR     NOT NULL,
//     //                 data    BYTEA
//     //             )"), &[]).ok().expect("Failed to create table \"person\""));
//
//     conn.execute("CREATE TABLE if not exists weekday (
//                     id      SERIAL      PRIMARY KEY,
//                     name    VARCHAR     NOT NULL,
//                     data    BYTEA
//                 )", &[]).ok().expect("Failed to create table \"person\"");
//
//     conn.execute("CREATE TABLE if not exists place (
//                     id      SERIAL      PRIMARY KEY,
//                     name    VARCHAR     NOT NULL,
//                     data    BYTEA
//                 )", &[]).ok().expect("Failed to create table \"person\"");
// }

// fn main() {
//     let mut db = env::var("DATABASE_URL").expect("No such env variable DATABASE_URL");
//
//     let conn = Connection::connect(db, TlsMode::None).unwrap();
//     init_db(&conn, true);
//
//     let me = Person {
//         id: 1,
//         name: "Steven".to_string(),
//         data: None,
//     };
//     conn.execute("INSERT INTO person (name, data) VALUES ($1, $2)",
//                  &[&me.name, &me.data]).unwrap();
//     for row in &conn.query("SELECT id, name, data FROM person", &[]).unwrap() {
//         let person = Person {
//             id: row.get(0),
//             name: row.get(1),
//             data: row.get(2),
//         };
//         println!("Found person {}", person.name);
//     }
// }
