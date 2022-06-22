use rusqlite::{Connection, Result};

pub fn get_connection() -> Result<Connection, rusqlite::Error> {
    let path = "./granger.db";
    Connection::open(&path)
}

pub fn inintialize_database() -> Result<()> {
    let connection = get_connection().unwrap();

    connection.execute(
        "CREATE TABLE IF NOT EXISTS board (
            id          INTEGER PRIMARY KEY,
            location    TEXT NOT NULL,
            name        BLOB
        )",
        [],
    )?;

    Ok(())
}
