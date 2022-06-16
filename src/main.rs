use clap::{ArgMatches, Command};

mod board_command;
use crate::board_command::{get_board_command, parse_board_options};

mod ticket_command;
use crate::ticket_command::{get_ticket_command, parse_ticket_options};

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
