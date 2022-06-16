use clap::{Arg, ArgMatches, Command};

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
        println!("INIT")
    }
    if options.is_present("list") {
        println!("LIST")
    }
    if options.is_present("remove") {
        println!("REMOVE")
    }
}
