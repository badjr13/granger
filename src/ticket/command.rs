use clap::{Arg, ArgMatches, Command};
use std::env;
use std::io::ErrorKind;

use crate::get_granger_data_directory;

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
    create_temporary_new_ticket_file();
    //     // let child_process = std::process::Command::new(editor)
    //     let child_process = std::process::Command::new("ls")
    //         .arg("/home/badjr13/workspaces")
    //         .stdout(std::process::Stdio::piped())
    //         .spawn()
    //         .expect("wow bob");

    //     let testaroo = child_process.wait_with_output().expect("wow bob 2");

    //     println!("{:?}", testaroo);
}

// {
//     Ok(ticket_template) => ticket_template,
//     Err(error) => match error.kind() {
//         ErrorKind::NotFound => panic!("Could not find editor provided."),
//         other_error => panic!("{:?}", other_error),
//     },
// };

fn create_temporary_new_ticket_file() {
    let granger_data_directory = get_granger_data_directory();

    let testaroo = std::process::Command::new("cp")
        .args([
            "/home/badjr13/workspaces/granger/src/ticket/template.toml",
            granger_data_directory.to_str().unwrap(),
        ])
        .output()
        .expect("oh no");

    println!("{:?}", testaroo)
}
