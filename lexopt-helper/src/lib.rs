pub mod prelude {
    pub use lexopt::prelude::*;
    pub use lexopt::Arg;
    pub use lexopt::Error;
    pub use lexopt::Parser as LexParser;

    use std::collections::HashMap;
    use std::ffi::OsString;

    pub struct ParserInfo {
        pub command_map: HashMap<String, DisplayCommand>,
        cmd_parser: LexParser,
    }

    impl ParserInfo {
        pub fn new() -> Self {
            ParserInfo {
                command_map: HashMap::new(),
                cmd_parser: LexParser::from_env(),
            }
        }

        pub fn next(&mut self) -> Result<Option<Arg<'_>>, Error> {
            self.cmd_parser.next()
        }

        pub fn value(&mut self) -> Result<OsString, Error> {
            self.cmd_parser.value()
        }
    }

    #[derive(Clone)]
    pub struct DisplayCommand {
        pub name: String,
        pub subcommands: Vec<DisplayCommand>,
        pub args: Vec<DisplayArg>,
        pub usage: String,
        pub description: String,
    }

    #[derive(Clone)]
    pub struct DisplayArg {
        pub optional: bool,
        pub long_name: String,
        pub short_name: String,
        pub description: String,
    }
}
