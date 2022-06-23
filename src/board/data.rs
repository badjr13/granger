use crate::board::model::Board;
use crate::database::get_connection;
use rusqlite::{params, Result};

pub fn add(board: Board) -> Result<()> {
    let connection = get_connection().unwrap();

    connection.execute(
        "
        INSERT INTO board (location, name)
        VALUES(?1, ?2)
        ",
        params![board.location.to_str(), board.name],
    )?;

    Ok(())
}
