use clap::Args;

#[derive(Args, Debug)]
pub struct TicketCommand {
    #[clap(short, long, display_order = 1)]
    /// Create a new ticket
    create: bool,

    #[clap(short, long, value_name = "ID", display_order = 2)]
    /// View details of an existing ticket
    read: usize,

    /// Update details of an existing ticket
    #[clap(short, long, value_name = "ID", display_order = 3)]
    update: usize,

    /// Delete an existing ticket
    #[clap(short, long, value_name = "ID", display_order = 4)]
    delete: usize,

    /// List all existing tickets in local board(current git repository)
    #[clap(short, long, value_name = "ID", display_order = 5)]
    list: bool,

    /// Move ticket to a different state in the local board(current git repository)
    #[clap(short, long, value_name = "ID", display_order = 6)]
    r#move: usize,
}

// OPTIONS:
//     -c, --create                Create a new ticket
//     -r, --read   <ID>           View details of an existing ticket
//     -u, --update <ID>           Update details on an existing ticket
//     -d, --delete <ID>           Delete an existing ticket

//     -ls, --list                 List all existing tickets in the current board
//     -mv, --move  <ID> <STATE>   Move ticket to a different state on the current board

// TEST ASD asdfasdfadsf
// Comment Bug: "ALL CAPS:" followed by a ":"
