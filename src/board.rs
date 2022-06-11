use clap::Args;

#[derive(Args, Debug)]
pub struct BoardCommand {
    #[clap(short, long, display_order = 1)]
    /// Initialize a board in local git repository
    init: bool,

    #[clap(short, long, display_order = 2)]
    /// List all boards on system
    list: bool,

    /// Remove board based on local git repository
    #[clap(short, long, display_order = 3)]
    remove: bool,
}
