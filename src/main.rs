use clap::{Arg, Command};

mod board;
use crate::board::get_board_command;

fn main() {
    let matches = Command::new("granger")
        .about("An opinionated Kanban Board for the solo developer.")
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand(get_board_command())
        .subcommand(
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
                ),
        )
        .get_matches();
}
