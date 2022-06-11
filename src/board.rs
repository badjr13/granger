use clap::Args;

#[derive(Args, Debug)]
pub struct BoardCommand {
    #[clap(short, long, display_order = 1)]
    /// Initialize a board in local git repository
    pub init: bool,

    #[clap(short, long, display_order = 2)]
    /// List all boards on system
    pub list: bool,

    /// Remove board based on local git repository
    #[clap(short, long, display_order = 3)]
    pub remove: bool,
}
