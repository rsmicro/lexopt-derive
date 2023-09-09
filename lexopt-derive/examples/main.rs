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

/*
impl CliArgs {
    pub fn parse_manual() -> Result<Self, Error> {
       parser.command_map.insert(
            "@".to_owned(),
            DisplayCommand {
                name: "ex".to_owned(),
                subcommands: vec![DisplayCommand {
                    name: "install".to_owned(),
                    subcommands: vec![],
                    args: vec![DisplayArg {
                        long_name: "name".to_owned(),
                        short_name: "n".to_owned(),
                        optional: true,
                        description: String::new(),
                    }],
                    usage: "Random usage".to_owned(),
                    description: "random description".to_owned(),
                }],
                args: vec![DisplayArg {
                    description: "verbose".to_owned(),
                    optional: true,
                    long_name: "verbose".to_owned(),
                    short_name: "v".to_owned(),
                }],
                usage: "ex [command] [--[options]]".to_owned(),
                description: "a description".to_owned(),
            },

        );
        while let Some(arg) = parser.next()? {
            match arg.clone() {
                Short('v') | Long("verbose") => {
                    let value: bool = parser.value()?.parse()?;
                    verbose = Some(value);
                }
                Short('h') | Long("help") => {
                    let cmd = parser.command_map.get("@").unwrap();
                    help(
                        Some(cmd.to_owned()),
                        cmd.subcommands.clone(),
                        cmd.args.clone(),
                    );
                    std::process::exit(0);
                }

                Value(value) => {
                    let val = value.as_os_str().to_str().unwrap();
                    command = Some(Command::parse(&mut parser, val)?)
                }
                _ => return Err(arg.unexpected()),
            }
        }
        Ok(CliArgs {
            command: command.unwrap(),
            verbose: verbose.unwrap_or_default(),
        })
    }
}
*/
#[derive(SubCommand, Default, Debug)]
pub enum Command {
    Install {
        name: String,
    },
    Hello {
        name: String,
    },
    #[default]
    Help,
}

impl Command {
    pub fn parse<T: Display + ?Sized>(parser: &mut ParserInfo, cmd_val: &T) -> Result<Self, Error> {
        parser.command_map.insert(
            "install".to_owned(),
            DisplayCommand {
                name: "install".to_owned(),
                subcommands: vec![],
                args: vec![DisplayArg {
                    long_name: "name".to_owned(),
                    short_name: "n".to_owned(),
                    optional: true,
                    description: String::new(),
                }],
                usage: "Random usage".to_owned(),
                description: "random description".to_owned(),
            },
        );
        match cmd_val.to_string().as_str() {
            "install" => Self::parse_install(parser),
            "hello" => Self::parse_hello(parser),
            _ => unreachable!(),
        }
    }

    fn parse_install(parser: &mut ParserInfo) -> Result<Self, Error> {
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

    fn parse_hello(_: &mut ParserInfo) -> Result<Self, Error> {
        unimplemented!()
    }
}

fn main() -> Result<(), Error> {
    let args = CliArgs::parse()?;
    println!("{:?}", args);
    Ok(())
}
