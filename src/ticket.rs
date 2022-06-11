use clap::Args;

#[derive(Args, Debug)]
pub struct TicketCommand {
    #[clap(short, long, display_order = 1)]
    /// Create a new ticket
    pub create: bool,

    #[clap(short, long, value_name = "ID", display_order = 2)]
    /// View details of an existing ticket
    pub read: Option<usize>,

    /// Update details of an existing ticket
    #[clap(short, long, value_name = "ID", display_order = 3)]
    pub update: Option<usize>,

    /// Delete an existing ticket
    #[clap(short, long, value_name = "ID", display_order = 4)]
    pub delete: Option<usize>,

    /// List all existing tickets in local board
    #[clap(short, long, display_order = 5)]
    pub list: bool,

    /// Move ticket to a different state in local board
    #[clap(short, long, value_names = &["ID", "STATE"], display_order = 6)]
    pub r#move: Option<usize>,
}
