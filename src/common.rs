use crate::board;
use std::env::current_dir;
use std::path::PathBuf;
use std::process;
use std::str::from_utf8;

pub fn get_root_path_if_git_repository(location: &PathBuf) -> Result<String, &'static str> {
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

pub fn get_git_repository_name(location: &str) -> String {
    // return everything after last "/" in path
    // example:
    //    /home/bobby/workspaces/granger -> granger
    let mut output: Vec<&str> = location.split('/').collect();
    output.pop().unwrap().to_string()
}

pub fn get_current_board() {
    let current_working_directory =
        current_dir().expect("Failure to get current working directory.");

    let path_to_git_repository =
        get_root_path_if_git_repository(&current_working_directory).unwrap();

    let board = board::data::get_by_location(path_to_git_repository).unwrap();
}
