use clap::{Parser, Subcommand};

mod board;
use crate::board::BoardCommand;

mod ticket;
use crate::ticket::TicketCommand;

#[derive(Debug, Subcommand)]
#[clap(disable_help_subcommand = true)]
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

    match &args.subcommands {
        GrangerSubcommand::Board(option) => {
            if option.init {
                println!("INIT")
            }
            if option.list {
                println!("LIST")
            }
            if option.remove {
                println!("REMOVE")
            }
        }
        // GrangerSubcommand::Ticket(x) => println!("TICKET: {:?}", x)
        GrangerSubcommand::Ticket(option) => {
            println!("{:?}", option);
            if option.create {
                println!("CREATE")
            }
            if option.read.is_some() {
                println!("READ")
            }
            if option.update.is_some() {
                println!("UPDATE")
            }
            if option.delete.is_some() {
                println!("DELETE")
            }
            if option.list {
                println!("LIST")
            }
            if option.r#move.is_some() {
                println!("MOVE")
            }
        }
    }
}
