use clap::{ArgMatches, Command};
use std::fs;
use std::path::PathBuf;

mod common;

mod database;
use crate::database::inintialize_database;

mod board;
use crate::board::command::{get_board_command, parse_board_options};

mod ticket;
use crate::ticket::command::{get_ticket_command, parse_ticket_options};

pub fn get_granger_data_directory() -> PathBuf {
    let granger_data_directory = dirs::data_dir()
        .expect("Failed to find granger data directory")
        .join("granger");

    fs::create_dir(&granger_data_directory).ok();

    granger_data_directory
}

fn main() {
    inintialize_database().unwrap();

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
                parse_board_options(options);
            }
        }
        Some(("ticket", options)) => {
            if options.args_present() {
                parse_ticket_options(options);
            }
        }
        _ => todo!(),
    }
}
