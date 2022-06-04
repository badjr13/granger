use clap::{arg, command, Command};

fn main() {
    let matches = command!()
        .arg(arg!(-b --board "Manage Boards"))
        .arg(arg!(-t --ticket "Manage Tickets"))
        .get_matches();
}

struct Board {
    name: String,
}

struct Ticket {
    acceptance_criteria: String,
    description: String,
    tags: Vec<Tag>,
    title: String,
    urgent: bool,
}

struct Tag {
    color: String,
    name: String,
}

enum State {
    ToDo { limit: isize },
    Blocked { limit: isize },
    Doing { limit: i8 },
    Review { limit: i8 },
    Complete { limit: isize },
}
