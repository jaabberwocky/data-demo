use rusqlite::{Connection, Result};

#[derive(Debug)]
#[allow(dead_code)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() -> Result<()> {
    let conn = Connection::open("data/sqlite.db")?;

    // drop table first
    conn.execute("DROP TABLE IF EXISTS person", ())?;
    conn.execute(
        "CREATE TABLE person (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            data  BLOB
        )",
        (), // empty list of parameters.
    )?;
    let me = Person {
        id: 0,
        name: "Tobias".to_string(),
        data: None,
    };
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        (&me.name, &me.data),
    )?;

    let person2 = Person {
        id: 1,
        name: "James".to_string(),
        data: Some((0..12).collect::<Vec<_>>()),
    };

    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        (&person2.name, &person2.data),
    )?;

    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
    Ok(())
}
