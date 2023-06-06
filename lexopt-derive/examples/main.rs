use lexopt_derive::{cli, help, Parser, SubCommand};
use lexopt_helper::prelude::{CLIFlag, CLiCommand};

#[derive(Parser)]
#[cli(
    name = "es",
    about = "Just another command to manage the command line arguments",
    version = "0.0.1",
    author = "Vincenzo Palazzo <vincenzopalazzo@member.fsf.org>"
)]
pub struct CliArgs {
    #[subcommand]
    pub install: Install,
    /// verbose flag
    pub verbose: bool,
}

#[derive(Debug, SubCommand)]
#[cli(about = "Install somethings, but we do not know what")]
pub struct Install {
    pub name: String,
}

#[help(CliArgs)]
fn help<C: CLiCommand, F: CLIFlag>(top_level: Option<C>, sucommands: Vec<C>, flags: Vec<F>) {
    if let Some(command) = top_level {
        println!("{}   {} ", command.name(), command.description());
        println!();
        println!("TODO add usage example");
    }
    for command in sucommands {
        println!();
        println!("  {}        {}", command.name(), command.description());
        help(None, command.subcommands::<C>(), command.flags::<F>());
    }

    for flag in flags {
        println!();
        println!();
        println!("-{} | --{}", flag.short(), flag.long());
    }
}

fn main() {
    println!("Hello, world!");
}
