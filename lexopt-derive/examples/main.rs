use lexopt_derive::{Parser, SubCommand};

#[derive(Parser)]
pub struct CliArgs {
    #[subcommand]
    pub install: Install,
    /// verbose flag
    pub verbose: bool,
}

#[derive(Debug, SubCommand)]
pub struct Install {
    pub name: String,
}

fn main() {
    println!("Hello, world!");
}
