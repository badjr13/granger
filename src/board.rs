use clap::Args;

#[derive(Args, Debug)]
pub struct BoardCommand {
    #[clap(long)]
    /// Initialize a board in current git repository
    init: bool,

    #[clap(long)]
    /// List of initalized boards on system
    list: bool,

    /// Remove board based in current git repository
    #[clap(long)]
    remove: bool,
}
