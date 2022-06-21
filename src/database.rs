use rusqlite::{Connection, Result};

pub fn inintialize_database() -> Result<()> {
    let path = "./granger.db";

    let conn = Connection::open(&path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS board (
            id          INTEGER PRIMARY KEY,
            location    TEXT NOT NULL,
            name        BLOB
        )",
        [],
    )?;

    Ok(())
}
