use clap::{arg, command, Command};
use std::path::PathBuf;

fn main() {
    let matches = command!()
        .arg(arg!(-b --board "Manage Boards"))
        .arg(arg!(-t --ticket "Manage Tickets"))
        .get_matches();
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

