use rusqlite::{Connection, Result};
use std::fs;
use std::fs::OpenOptions;
use std::path::PathBuf;

pub fn get_granger_db_directory() -> PathBuf {
    dirs::data_dir()
        .expect("Failed to find laydown config directory")
        .join("granger")
}

pub fn get_path_to_db() -> PathBuf {
    let granger_db_directory = get_granger_db_directory();
    fs::create_dir(&granger_db_directory).ok();

    let granger_db_file: PathBuf = granger_db_directory.join("granger.db");

    OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(&granger_db_file)
        .expect("Failed to find laydown.ron file");

    granger_db_file
}

pub fn get_connection() -> Result<Connection, rusqlite::Error> {
    Connection::open(&get_path_to_db())
}

pub fn inintialize_database() -> Result<()> {
    let connection = get_connection()?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS board (
            id          INTEGER PRIMARY KEY,
            name        TEXT NOT NULL,
            location    TEXT NOT NULL UNIQUE
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS ticket (
            id          INTEGER PRIMARY KEY,
            board_id    INTEGER,
            title       TEXT NOT NULL,
            description TEXT NOT NULL,
            state       TEXT NOT NULL
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS state (
            todo_count    INTEGER,
            blocked_count INTEGER,
            doing_count   INTEGER,
            review_count  INTEGER,
            done_count    INTEGER,
            urgent_count  INTEGER

        )",
        [],
    )?;

    Ok(())
}
