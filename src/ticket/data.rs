use crate::database::get_connection;
use crate::ticket::model::Ticket;
use rusqlite::{params, Result};

pub fn add(ticket: &Ticket) -> Result<()> {
    let connection = get_connection()?;

    connection.execute(
        "
        INSERT INTO ticket (board_id, title, description, state)
        VALUES(?1, ?2, ?3, ?4);
        ",
        params![
            ticket.board_id,
            ticket.title,
            ticket.description,
            ticket.state.get(),
        ],
    )?;

    Ok(())
}

pub fn get_by_id(id: usize) -> Result<Ticket> {
    let connection = get_connection()?;

    let mut statement = connection.prepare("SELECT * FROM ticket WHERE id=?;")?;

    let row = statement.query_row(params![id], |row| {
        Ok(Ticket {
            id: row.get(0)?,
            board_id: row.get(1)?,
            title: row.get(2)?,
            description: row.get(3)?,
            state: row.get(4)?,
        })
    })?;

    Ok(row)
}
