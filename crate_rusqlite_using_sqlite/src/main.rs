extern crate rusqlite;
extern crate time;

use rusqlite::{params, Connection, Result};
use time::Timespec;

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    time_created: Timespec,
    data: Option<Vec<u8>>,
}

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  time_created    TEXT NOT NULL,
                  data            BLOB
                  )",
        params![],
    )?;
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        time_created: time::get_time(),
        data: None,
    };
    conn.execute(
        "INSERT INTO person (name, time_created, data)
                  VALUES (?1, ?2, ?3)",
        params![me.name, me.time_created, me.data],
    )?;

    let mut stmt = conn.prepare("SELECT id, name, time_created, data FROM person")?;
    let person_iter = stmt.query_map(params![], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            time_created: row.get(2)?,
            data: row.get(3)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
    Ok(())
}
