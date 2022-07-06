use crate::get_granger_data_directory;
use clap::{Arg, ArgMatches, Command};
use std::env;
use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::path::PathBuf;

pub fn get_ticket_command() -> Command<'static> {
    Command::new("ticket")
        .about("Manage Tickets")
        .arg(
            Arg::new("create")
                .short('c')
                .long("create")
                .help("Create a new ticket")
                .display_order(1),
        )
        .arg(
            Arg::new("read")
                .short('r')
                .long("read")
                .help("View details of an existing ticket")
                .display_order(2),
        )
        .arg(
            Arg::new("update")
                .short('u')
                .long("update")
                .help("Update details of an existing ticket")
                .display_order(3),
        )
        .arg(
            Arg::new("delete")
                .short('d')
                .long("delete")
                .help("Delete an existing ticket")
                .display_order(4),
        )
        .arg(
            Arg::new("list")
                .short('l')
                .long("list")
                .help("List all existing tickets in local board")
                .display_order(5),
        )
        .arg(
            Arg::new("move")
                .short('m')
                .long("move")
                .help("Move ticket to a different state in local board")
                .display_order(6),
        )
}

pub fn parse_ticket_options(options: &ArgMatches) {
    if options.is_present("create") {
        let default_editor = env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
        create_new_ticket(default_editor);
    }
    if options.is_present("read") {
        println!("READ")
    }
    if options.is_present("update") {
        println!("UPDATE")
    }
    if options.is_present("delete") {
        println!("DELETE")
    }
    if options.is_present("list") {
        println!("LIST")
    }
    if options.is_present("move") {
        println!("MOVE")
    }
}

fn create_new_ticket(editor: String) {
    let new_ticket_template = get_new_ticket_template();

    match std::process::Command::new(editor)
        .arg(new_ticket_template)
        .status()
    {
        Ok(ticket_template) => ticket_template,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("Could not find editor provided."),
            other_error => panic!("{:?}", other_error),
        },
    };
}

fn get_new_ticket_template() -> PathBuf {
    let granger_db_directory = get_granger_data_directory();

    let new_ticket_template = granger_db_directory.join(".ticket_template.toml");

    OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(&new_ticket_template)
        .expect("Failed to find laydown.ron file");

    new_ticket_template
}
