use kproc_parser::kparser::KParserTracer;
use kproc_parser::proc_macro::TokenStream;
use kproc_parser::rust::kparser::RustParser;

mod cli;
mod help;
mod parser;

use cli as cli_parser;

struct Tracer;

impl KParserTracer for Tracer {
    fn log(&self, msg: &str) {
        eprintln!("|_ {msg}\n");
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

#[proc_macro_attribute]
pub fn cli(attribute: TokenStream, item: TokenStream) -> TokenStream {
    let parser = RustParser::new();
    // the item is alwayt on a parser struct.
    let ast = parser.parse_struct(&item);
    cli_parser::parse(attribute, ast, item)
}

#[proc_macro_attribute]
pub fn help(attribute: TokenStream, item: TokenStream) -> TokenStream {
    help::parse(attribute, item)
}
