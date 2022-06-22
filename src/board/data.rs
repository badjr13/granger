use crate::board::model::Board;
use crate::database::get_connection;
use rusqlite::{params, Result};

pub fn add(board: Board) -> Result<()> {
    let connection = get_connection().unwrap();

    connection.execute(
        "
        INSERT INTO board (id, location, name)
        VALUES(?1, ?2, ?3)
        ",
        params![board.id, board.location.to_str(), board.name],
    )?;

    Ok(())
}
