use clap::{Parser, Subcommand};

mod board;
use crate::board::BoardCommand;

mod ticket;
use crate::ticket::TicketCommand;

#[derive(Debug, Subcommand)]
enum GrangerSubcommand {
    /// Manage Boards
    Board(BoardCommand),

    /// Manage Tickets
    Ticket(TicketCommand),
}

#[derive(Debug, Parser)]
#[clap(about, author, version)]
struct GrangerArguments {
    #[clap(subcommand)]
    subcommands: GrangerSubcommand,
}

fn main() {
    let args = GrangerArguments::parse();
}
