use clap::{arg, ArgMatches, command, Command};

fn main() {
    let matches = command!()
        .disable_help_subcommand(true)
        .subcommand(
            Command::new("board")
            .about("Initialize, List, and Remove Boards")
            .arg(arg!(-i --init "Initialize a board in current git repository"))
            .arg(arg!(-l --list "List of initalized boards on system"))
            .arg(arg!(-r --remove "Remove board based on current git repository"))
        )
        .subcommand(
            Command::new("ticket")
            .about("Create, Read, Update, and Delete Tickets")
            .arg(arg!(-c --create "Create a new ticket"))
            .arg(arg!(-r --read "View details of an existing ticket"))
            .arg(arg!(-u --update "Update details on an existing ticket"))
            .arg(arg!(-d --delete "Delete an existing ticket"))
            .arg(arg!(-l --list "List all existing tickets in the current board"))
            .arg(arg!(-m --move "Move ticket to a different state on the current board"))
        )
        .get_matches();

    run(matches)
}

fn run(matches: ArgMatches) {
    match matches.subcommand() {
        Some(("board", board_args)) => println!("BOARD"),
        Some(("ticket", wow)) => println!("TICKET"),
        _ => println!("NOTHING")
    }
}

