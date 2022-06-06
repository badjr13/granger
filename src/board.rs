use std::path::PathBuf;
use clap::ArgMatches;

pub fn run(board_matches: &ArgMatches) {
    println!("{:?}", board_matches);
}


struct Board {
    name: String,
    location: PathBuf,
}

