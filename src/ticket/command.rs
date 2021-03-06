use crate::common;
use crate::ticket;
use crate::ticket::model::{State, Ticket};
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
                .takes_value(true)
                .value_name("ticket id")
                .short('r')
                .long("read")
                .help("View details of an existing ticket")
                .display_order(2),
        )
        .arg(
            Arg::new("update")
                .takes_value(true)
                .value_name("ticket id")
                .short('u')
                .long("update")
                .help("Update details of an existing ticket")
                .display_order(3),
        )
        .arg(
            Arg::new("delete")
                .takes_value(true)
                .value_name("ticket id")
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
                .takes_value(true)
                .number_of_values(2)
                .value_names(&["id", "state"])
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
        let ticket_id: usize = options.value_of("read").unwrap().parse().unwrap();
        let ticket = ticket::data::get_by_id(ticket_id).unwrap();
        println!("{:?}", ticket);
    }

    if options.is_present("update") {
        let ticket_id: usize = options.value_of("update").unwrap().parse().unwrap();
        update_ticket(ticket_id);
    }

    if options.is_present("delete") {
        let ticket_id: usize = options.value_of("delete").unwrap().parse().unwrap();
        ticket::data::delete(ticket_id).unwrap();
        println!("Ticket with id of '{}' deleted successfully.", ticket_id);
    }

    if options.is_present("list") {
        let tickets = ticket::data::get_all().unwrap();
        println!("{:#?}", tickets);
    }

    if options.is_present("move") {
        let arguments: Vec<_> = options.values_of("move").unwrap().collect();
        let ticket_id = arguments[0].parse().unwrap();
        let state = State::from_string(arguments[1]);
        ticket::data::move_state(ticket_id, state.value()).unwrap();
    }
}

fn create_new_ticket() {
    create_ticket_file();

    open_interactive_ticket_file();

    let (title, description) = process_ticket_data();

    let current_board_id = common::get_current_board().id;

    let ticket = Ticket::new(current_board_id, title, description);

    match ticket::data::add(&ticket) {
        Ok(_) => println!("'{}' ticket created.", ticket.title),
        Err(value) => println!("Error creating '{}' ticket : {}", ticket.title, value),
    }

    remove_ticket_file();
}

fn update_ticket(ticket_id: usize) {
    let ticket = ticket::data::get_by_id(ticket_id).unwrap();
    let ticket_toml_string = toml::to_string(&ticket).unwrap();

    create_ticket_file();

    write_to_ticket_file(ticket_toml_string);

    open_interactive_ticket_file();

    let (title, description) = process_ticket_data();

    ticket::data::update(ticket_id, title, description).unwrap();
}

fn create_ticket_file() {
    // Need to figure out how to include template file in binary
    std::process::Command::new("cp")
        .args([
            "/home/badjr13/workspaces/granger/src/ticket_template.toml",
            env::temp_dir().to_str().unwrap(),
        ])
        .output()
        .expect("oh no");
}

fn get_ticket_file() -> String {
    format!("{}/ticket_template.toml", env::temp_dir().to_str().unwrap(),)
}

fn open_interactive_ticket_file() {
    let editor = env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());

    let temp_file = get_ticket_file();

    match std::process::Command::new(editor).arg(temp_file).status() {
        Ok(ticket_template) => ticket_template,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("Could not find editor provided."),
            other_error => panic!("{:?}", other_error),
        },
    };
}

fn process_ticket_data() -> (String, String) {
    let temp_file = get_ticket_file();
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

fn remove_ticket_file() {
    let temp_file = get_ticket_file();

    std::process::Command::new("rm")
        .arg(temp_file)
        .status()
        .unwrap();
}

fn write_to_ticket_file(data: String) {
    let file = get_ticket_file();
    fs::write(file, data).unwrap();
}
