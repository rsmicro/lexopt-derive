pub mod prelude {
    pub use lexopt::prelude::*;
    pub use lexopt::*;

    pub trait HelpePrinter {
        fn print_help(&self) -> String;
    }

    pub trait CLiCommand {
        fn name(&self) -> String;
        fn description(&self) -> String;
        fn usage(&self) -> Option<String>;
    }

    pub trait CLIFlag {
        fn long(&self) -> String;
        fn short(&self) -> String;
    }
}
