use ::std::path::PathBuf;
use clap::Parser;

fn main() {
    let args = Board::parse();
    println!("{:?}", args)
}

#[derive(Parser, Debug)]
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
