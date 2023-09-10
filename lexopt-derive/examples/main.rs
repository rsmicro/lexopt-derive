use std::fmt::Display;

use lexopt_derive::cli;
use lexopt_derive::{Parser, SubCommand};
use lexopt_helper::prelude::*;

fn help(
    top_level: Option<DisplayCommand>,
    sucommands: Vec<DisplayCommand>,
    flags: Vec<DisplayArg>,
) {
    if let Some(command) = top_level {
        println!("{}   {} ", command.name, command.description);
        println!();
        println!("TODO add usage example");
    }
    for command in sucommands {
        println!();
        println!("  {}        {}", command.name, command.description);
        help(None, command.subcommands, command.args.clone());
    }

    for flag in flags {
        println!();
        println!();
        println!("-{} | --{}", flag.long_name, flag.short_name);
    }
}

#[derive(Parser, Debug)]
#[cli(
    name = "es",
    about = "Just another command to manage the command line arguments",
    version = "0.0.1",
    author = "Vincenzo Palazzo <vincenzopalazzo@member.fsf.org>"
)]
pub struct CliArgs {
    #[subcommand]
    pub command: Command,
    /// verbose flag
    pub verbose: bool,
}

#[derive(SubCommand, Debug)]
pub enum Command {
    Install { name: String },
    Hello { name: String },
}

impl Command {
    fn parse_install_old(parser: &mut ParserInfo) -> Result<Self, Error> {
        let mut name: Option<String> = None;
        while let Some(arg) = parser.next()? {
            match arg.clone() {
                Short('n') | Long("name") => {
                    let value: String = parser.value()?.parse()?;
                    name = Some(value);
                }
                Short('h') | Long("help") => {
                    let cmd = parser.command_map.get("install").unwrap();
                    help(
                        Some(cmd.to_owned()),
                        cmd.subcommands.clone(),
                        cmd.args.clone(),
                    );
                    std::process::exit(0);
                }
                _ => return Err(arg.unexpected()),
            }
        }
        Ok(Self::Hello {
            name: name.unwrap_or_default(),
        })
    }
}

fn main() -> Result<(), Error> {
    let args = CliArgs::parse()?;
    println!("{:?}", args);
    Ok(())
}
