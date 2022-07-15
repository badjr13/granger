use crate::board;
use crate::board::model::Board;
use crate::common;
use clap::{Arg, ArgMatches, Command};
use std::env::current_dir;

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
            common::get_root_path_if_git_repository(&current_working_directory).unwrap();

        let git_repository_name = common::get_git_repository_name(&path_to_git_repository);

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
        let current_working_directory =
            current_dir().expect("Failure to get current working directory.");

        let path_to_git_repository =
            common::get_root_path_if_git_repository(&current_working_directory).unwrap();

        let board = board::data::get_by_location(path_to_git_repository).unwrap();

        match board::data::remove(board.id.try_into().unwrap()) {
            Ok(_) => println!("{:?} board successfully removed.", board.name),
            Err(value) => println!("{:?}", value),
        }
    }
}
