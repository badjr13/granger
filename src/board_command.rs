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
        if is_git_repository(location) {
            todo!() // initialize a new board
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

fn is_git_repository(location: PathBuf) -> bool {
    let output = process::Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .current_dir(location)
        .output();

    if let Ok(value) = output {
        if from_utf8(&value.stdout) == Ok("true\n") {
            true
        } else {
            false
        }
    } else {
        false
    }
}
