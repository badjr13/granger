use clap::{Arg, ArgMatches, Command};
use std::env;
use std::fs;
use std::io::ErrorKind;

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
        create_new_ticket();
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

fn create_new_ticket() {
    create_temporary_new_ticket_file();

    open_temporary_new_ticket_file();

    let (title, description) = process_new_ticket_data();

    remove_temporary_new_ticket_file();
}

fn create_temporary_new_ticket_file() {
    // Need to figure out how to include template file in binary
    std::process::Command::new("cp")
        .args([
            "/home/badjr13/workspaces/granger/src/ticket/granger_ticket_template.toml",
            env::temp_dir().to_str().unwrap(),
        ])
        .output()
        .expect("oh no");
}

fn get_temporary_new_ticket_file() -> String {
    format!(
        "{}/granger_ticket_template.toml",
        env::temp_dir().to_str().unwrap(),
    )
}

fn open_temporary_new_ticket_file() {
    let editor = env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());

    let temp_file = get_temporary_new_ticket_file();

    match std::process::Command::new(editor).arg(temp_file).status() {
        Ok(new_ticket_template) => new_ticket_template,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("Could not find editor provided."),
            other_error => panic!("{:?}", other_error),
        },
    };
}

fn process_new_ticket_data() -> (String, String) {
    let temp_file = get_temporary_new_ticket_file();
    let content = fs::read_to_string(temp_file)
        .expect("Failed to read content from new ticket template file.");

    let deserialized_content: toml::Value = match toml::from_str(&content) {
        Ok(_content) => _content,
        Err(error) => panic!(
            "Error deserializing data from new ticket template: {}",
            error
        ),
    };

    let title = deserialized_content
        .get("title")
        .unwrap()
        .as_str()
        .unwrap()
        .trim()
        .to_string();

    let description = deserialized_content
        .get("description")
        .unwrap()
        .as_str()
        .unwrap()
        .trim()
        .to_string();

    (title, description)
}

fn remove_temporary_new_ticket_file() {
    let temp_file = get_temporary_new_ticket_file();

    std::process::Command::new("rm")
        .arg(temp_file)
        .status()
        .unwrap();
}
