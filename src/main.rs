use clap::{ArgMatches, Command};

mod board;
use crate::board::get_board_command;

mod ticket;
use crate::ticket::get_ticket_command;

fn main() {
    let matches = Command::new("granger")
        .about("An opinionated Kanban Board for the solo developer.")
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand(get_board_command())
        .subcommand(get_ticket_command())
        .get_matches();

    run(matches)
}

fn run(args: ArgMatches) {
    todo!()
}
