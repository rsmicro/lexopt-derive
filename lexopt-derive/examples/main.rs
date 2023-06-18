use lexopt_derive::{cli, help, Parser, SubCommand};
use lexopt_helper::prelude::*;

#[derive(Parser, Debug, Default, Clone)]
#[cli(
    name = "es",
    about = "Just another command to manage the command line arguments",
    version = "0.0.1",
    author = "Vincenzo Palazzo <vincenzopalazzo@member.fsf.org>"
)]
pub struct CliArgs {
    #[subcommand]
    pub install: Option<Install>,
    /// verbose flag
    pub verbose: bool,
}

impl CliArgs {
    pub fn parse() -> Result<Self, Error> {
        let mut install: Option<Install> = None;
        let mut verbose: Option<bool> = None;

        let mut parser = Parser::from_env();
        while let Some(arg) = parser.next()? {
            match arg.clone() {
                Short('v') | Long("verbose") => {
                    let value: bool = parser.value()?.parse()?;
                    verbose = Some(value);
                }
                Short('h') | Long("help") => {
                    let cmd = Self::default();
                    help(Some(cmd.clone()), cmd.subcommands(), cmd.flags::<String>());
                    std::process::exit(0);
                }

                Value(value) => {
                    let val = value.as_os_str().to_str().unwrap();
                    match val {
                        "install" => install = Some(Install::parse(&mut parser)?),
                        _ => return Err(arg.unexpected()),
                    }
                }
                _ => return Err(arg.unexpected()),
            }
        }
        Ok(CliArgs {
            install,
            verbose: verbose.unwrap_or_default(),
        })
    }
}

// FIXME: this should be how we show declare
// the subcommand
enum Command {
    Install { name: String },
    Hello { name: String },
}

#[derive(Debug, SubCommand, Default, Clone)]
#[cli(about = "Install somethings, but we do not know what")]
pub struct Install {
    pub name: String,
}

impl Install {
    pub fn parse(parser: &mut Parser) -> Result<Self, Error> {
        let mut name: Option<String> = None;
        while let Some(arg) = parser.next()? {
            match arg {
                Short('n') | Long("name") => {
                    let value: String = parser.value()?.parse()?;
                    name = Some(value);
                }
                Short('h') | Long("help") => {
                    let cmd = Self::default();
                    help(Some(cmd.clone()), cmd.subcommands(), cmd.flags::<String>());
                    std::process::exit(0);
                }
                _ => return Err(arg.unexpected()),
            }
        }
        Ok(Self {
            name: name.expect("the name need to be specified"),
        })
    }
}

#[help(CliArgs)]
fn help<C: CLiDescription, F: CLIFlag>(top_level: Option<C>, sucommands: Vec<C>, flags: Vec<F>) {
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

fn main() -> Result<(), Error> {
    let args = CliArgs::parse()?;
    println!("{:?}", args);
    Ok(())
}
