use ::std::path::PathBuf;
use clap::{Parser, Subcommand};

fn main() {
    let args = Cli::parse();
    println!("{:?}", args)
}

#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    board,
    ticket,
}

struct Board {
    name: String,
    location: PathBuf,
}

struct Ticket {
    title: String,
    description: String,
    acceptance_criteria: String,
}

enum State {
    ToDo { limit: isize },
    Blocked { limit: isize },
    Doing { limit: i8 },
    Review { limit: i8 },
    Complete { limit: isize },
}
