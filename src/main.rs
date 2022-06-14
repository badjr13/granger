use clap::{ArgMatches, Command};

mod board;
use crate::board::get_board_command;

mod ticket;
use crate::ticket::get_ticket_command;

fn main() {
    let matches = Command::new("granger")
        .about("An opinionated Kanban Board for the solo developer.")
        .disable_help_subcommand(true)
        .arg_required_else_help(true)
        .subcommand(get_board_command())
        .subcommand(get_ticket_command())
        .get_matches();

    parse_subcommands(matches)
}

fn parse_subcommands(args: ArgMatches) {
    match args.subcommand() {
        Some(("board", options)) => {
            if options.args_present() {
                parse_board_options(options)
            }
        }
        Some(("ticket", options)) => {
            if options.args_present() {
                parse_ticket_options(options)
            }
        }
        _ => todo!(),
    }
}

fn parse_board_options(options: &ArgMatches) {
    if options.is_present("init") {
        println!("INIT")
    }
    if options.is_present("list") {
        println!("LIST")
    }
    if options.is_present("remove") {
        println!("REMOVE")
    }
}

fn parse_ticket_options(options: &ArgMatches) {
    if options.is_present("create") {
        println!("CREATE")
    }
    if options.is_present("read") {
        println!("READ")
    }
    if options.is_present("update") {
        println!("UPDATE")
    }
    if options.is_present("delete") {
        println!("DELETE")
    }
    if options.is_present("list") {
        println!("LIST")
    }
    if options.is_present("move") {
        println!("MOVE")
    }
}
