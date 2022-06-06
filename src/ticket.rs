use clap::ArgMatches;

pub fn run(ticket_matches: &ArgMatches) {
    println!("{:?}", ticket_matches);
}

struct Ticket {
    title: String,
    description: String,
    acceptance_criteria: String,
}

