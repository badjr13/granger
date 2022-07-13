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
        params![ticket.board_id, ticket.title, ticket.description, "todo",],
    )?;

    Ok(())
}
