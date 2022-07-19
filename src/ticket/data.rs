use crate::database::get_connection;
use crate::ticket::model::{State, Ticket};
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
            ticket.state.value(),
        ],
    )?;

    Ok(())
}

pub fn get_by_id(id: usize) -> Result<Ticket> {
    let connection = get_connection()?;

    let mut statement = connection.prepare("SELECT * FROM ticket WHERE id=?;")?;

    let row = statement.query_row(params![id], |row| {
        let state: String = row.get(4)?;
        let state_as_str: &str = state.as_str();

        Ok(Ticket {
            id: row.get(0)?,
            board_id: row.get(1)?,
            title: row.get(2)?,
            description: row.get(3)?,
            state: State::from_string(state_as_str),
        })
    })?;

    Ok(row)
}

pub fn get_all() -> Result<Vec<Ticket>> {
    let connection = get_connection()?;

    let mut statement = connection.prepare("SELECT * FROM ticket;")?;

    let ticket_iter = statement.query_map([], |row| {
        let state: String = row.get(4)?;
        let state_as_str: &str = state.as_str();

        Ok(Ticket {
            id: row.get(0)?,
            board_id: row.get(1)?,
            title: row.get(2)?,
            description: row.get(3)?,
            state: State::from_string(state_as_str),
        })
    })?;

    let mut tickets = Vec::new();

    for ticket_result in ticket_iter {
        match ticket_result {
            Ok(ticket) => tickets.push(ticket),
            Err(value) => println!("Error while fetching tickets: {}", value),
        }
    }

    Ok(tickets)
}
