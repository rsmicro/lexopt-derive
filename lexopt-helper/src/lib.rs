pub mod prelude {
    pub use lexopt::prelude::*;
    pub use lexopt::*;

    pub trait CLiDescription {
        fn name(&self) -> String;
        fn description(&self) -> String;
        fn usage(&self) -> Option<String>;
        fn subcommands<C: CLiDescription>(&self) -> Vec<C>;
        fn flags<F: CLIFlag>(&self) -> Vec<F>;
    }

    pub trait CLIFlag {
        fn long(&self) -> String;
        fn short(&self) -> String;
    }

    impl CLIFlag for String {
        fn long(&self) -> String {
            unimplemented!()
        }

        fn short(&self) -> String {
            unimplemented!()
        }
    }
}
