use clap::{Parser, Subcommand};
mod board;
mod ticket;

fn main() {
    let args = Cli::parse();
    match args.subcommands {
        GrangerSubcommand::Board => board::run(),
        GrangerSubcommand::Ticket => ticket::run(),
    }
}

#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    subcommands: GrangerSubcommand,
}

#[derive(Subcommand, Debug)]
enum GrangerSubcommand {
    /// Manage Boards
    Board,
    /// Manage Tickets
    Ticket,
}
