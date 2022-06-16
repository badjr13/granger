use crate::board::Board;
use clap::{Arg, ArgMatches, Command};
use std::env::current_dir;
use std::path::PathBuf;
use std::process;
use std::str::from_utf8;

pub fn get_board_command() -> Command<'static> {
    Command::new("board")
        .about("Manage Boards")
        .arg(
            Arg::new("init")
                .short('i')
                .long("init")
                .help("Initialize a board in local git repository"),
        )
        .arg(
            Arg::new("list")
                .short('l')
                .long("list")
                .help("List all boards on system"),
        )
        .arg(
            Arg::new("remove")
                .short('r')
                .long("remove")
                .help("Remove board based on local git repository"),
        )
}

pub fn parse_board_options(options: &ArgMatches) {
    if options.is_present("init") {
        let location = current_dir().expect("Failure to get current working directory.");
        if get_root_if_git_repository(&location) {
            println!("You've made it this far!");
            // Board::new(location);
        } else {
            eprintln!("Boards must be initialized inside of a git repository.");
        }
    }
    if options.is_present("list") {
        println!("LIST");
    }
    if options.is_present("remove") {
        println!("REMOVE");
    }
}

// Refactor to return git repo root location to be passed to Board::new(location)
fn get_root_if_git_repository(location: &PathBuf) -> bool {
    let output = process::Command::new("git")
        .current_dir(location)
        .args(["rev-parse", "--show-toplevel"])
        .output();

    if let Ok(value) = output {
        if value.stdout.is_empty() {
            false
        } else {
            true
        }
    } else {
        false
    }
}
