use crate::board;
use crate::board::model::Board;
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
        let current_working_directory =
            current_dir().expect("Failure to get current working directory.");

        let path_to_git_repository =
            get_root_path_if_git_repository(&current_working_directory).unwrap();

        let git_repository_name = get_git_repository_name(&path_to_git_repository);

        let board = Board::new(&git_repository_name, &path_to_git_repository);

        match board::data::add(board) {
            Ok(_) => println!(
                "'{}' board has been created at '{}'.",
                git_repository_name, path_to_git_repository
            ),
            Err(value) => println!(
                "Error creating '{}' board at '{}': {}",
                git_repository_name, path_to_git_repository, value
            ),
        }
    }
    if options.is_present("list") {
        let boards = board::data::get_all();

        if let Ok(board) = boards {
            // Update to use display::fmt or board names only
            println!("{:?}", board);
        }
    }
    if options.is_present("remove") {
        println!("REMOVE");
    }
}

fn get_root_path_if_git_repository(location: &PathBuf) -> Result<String, &'static str> {
    // "git rev-parse --show-toplevel" returns the path to the root
    // of a git repository if called anywhere inside git repository
    let output = process::Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(location)
        .output();

    match output {
        Ok(value) => {
            if value.stdout.is_empty() {
                Err("Boards must be initialized inside of a git repository.")
            } else {
                let root_as_string = from_utf8(&value.stdout).unwrap();
                Ok(String::from(root_as_string.trim()))
            }
        }
        Err(_) => Err("Unreachable?"),
    }
}

fn get_git_repository_name(location: &str) -> String {
    // return everything after last "/" in path
    // example:
    //    /home/bobby/workspaces/granger -> granger
    let mut output: Vec<&str> = location.split('/').collect();
    output.pop().unwrap().to_string()
}
