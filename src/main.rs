use clap::{Parser, Subcommand};

mod board;
use crate::board::BoardCommand;

#[derive(Debug, Subcommand)]
enum GrangerSubcommand {
    /// Manage Boards
    Board(BoardCommand),
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
