pub mod prelude {
    pub use lexopt::prelude::*;
    pub use lexopt::*;

    pub trait CLiCommand {
        fn name(&self) -> String;
        fn description(&self) -> String;
        fn usage(&self) -> Option<String>;
        fn subcommands<C: CLiCommand>(&self) -> Vec<C>;
        fn flags<F: CLIFlag>(&self) -> Vec<F>;
    }

    pub trait CLIFlag {
        fn long(&self) -> String;
        fn short(&self) -> String;
    }
}
