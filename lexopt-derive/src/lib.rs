use kproc_parser::kparser::KParserTracer;
use kproc_parser::proc_macro::TokenStream;

mod parser;

struct Tracer;

impl KParserTracer for Tracer {
    fn log(&self, msg: &str) {
        eprintln!("{msg}");
    }
}

#[proc_macro_derive(Parser, attributes(subcommand))]
pub fn parser(tokens: TokenStream) -> TokenStream {
    parser::parse(tokens)
}

#[proc_macro_derive(SubCommand, attributes(subcommand))]
pub fn subcommand(tokens: TokenStream) -> TokenStream {
    parser::parse(tokens)
}
