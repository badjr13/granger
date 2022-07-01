use crate::board::model::Board;
use crate::database::get_connection;
use rusqlite::{params, Result};

pub fn add(board: Board) -> Result<()> {
    let connection = get_connection()?;

    connection.execute(
        "
        INSERT INTO board (name, location)
        VALUES(?1, ?2);
        ",
        params![board.name, board.location],
    )?;

    Ok(())
}

pub fn get_all() -> Result<Vec<Board>> {
    let connection = get_connection()?;

    let mut statement = connection.prepare(
        "
        SELECT * FROM board;
        "
    )?;

    let board_iter = statement.query_map([], |row| {
        Ok(Board {
            id: row.get(0)?,
            name: row.get(1)?,
            location: row.get(2)?,
        })
    })?;

    let mut boards = Vec::new();

    for board_result in board_iter {
        match board_result {
            Ok(board) => boards.push(board),
            Err(value) => println!("Error while fetching boards: {}", value),
        }
    }

    Ok(boards)
}

pub fn get_by_location(location: String) -> Result<Board> {
    let connection = get_connection()?;

    let mut statement = connection.prepare(
        "
        SELECT * FROM board WHERE location=?;
        "
    )?;

    let row = statement.query_row(params![location], |row| {
        Ok(Board {
            id: row.get(0)?,
            name: row.get(1)?,
            location: row.get(2)?,
        })
    })?;

    Ok(row)
}

pub fn remove(board_id: u8) -> Result<()> {
    let connection = get_connection()?;

    connection.execute(
        "
        DELETE FROM board where id=?1;
        ",
        params![board_id],
    )?;

    Ok(())
}
