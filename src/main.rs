use clap::{arg, command, Command};

fn main() {
    let matches = command!()
        .disable_help_subcommand(true)
        .subcommand(Command::new("board"))
        .subcommand(Command::new("ticket"))
        .get_matches();
}

