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
            // erase database
            self.conn.execute("
                DROP SCHEMA public CASCADE;
                CREATE SCHEMA public;
                GRANT ALL ON SCHEMA public TO postgres;
                GRANT ALL ON SCHEMA public TO public;
                COMMENT ON SCHEMA public IS 'standard public> schema';", &[]).ok().unwrap();
        }

        self.conn.execute(
            "CREATE TABLE if not exists place (id  VARCHAR  PRIMARY KEY)", &[]).ok().expect("Failed to create table \"place\"");

        self.conn.execute(
            "CREATE TABLE if not exists weekday (id  INTEGER  PRIMARY KEY)", &[]).ok().expect("Failed to create table \"weekday\"");

        for day in 0..7 {
            println!("Insert {:?}", day);
            self.conn.execute(
                "INSERT INTO weekday (id) VALUES ($1)",
                &[&day]).ok();
        }
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
    let mut db = env::var("DATABASE_URL").expect("No such env variable DATABASE_URL");

    let conn = Connection::connect(db, TlsMode::None).unwrap();
    let db = Database::new(&conn);
    db.init(true);
    println!("{:?}", db.get());
    // db.lalala();
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
