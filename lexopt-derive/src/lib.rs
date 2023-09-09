use kproc_parser::kparser::KParserTracer;
use kproc_parser::proc_macro::TokenStream;
use kproc_parser::rust::kparser::RustParser;

mod cli;
mod help;
mod parser;

use cli as cli_parser;

const TRACER: Tracer = Tracer {};

struct Tracer;

impl KParserTracer for Tracer {
    fn log(&self, msg: &str) {
        eprintln!("\x1b[93mproc_macro\x1b[0m: {msg}");
    }
}

#[proc_macro_derive(Parser, attributes(subcommand))]
pub fn parser(tokens: TokenStream) -> TokenStream {
    parser::parse(tokens)
}

#[proc_macro_derive(SubCommand, attributes(subcommand))]
pub fn subcommand(tokens: TokenStream) -> TokenStream {
    "".parse().unwrap()
}

/// cli procedural macro attribute
///
/// EXPAND:
/// ```ignore
/// pub struct ParserInfo {
///     name: String,
///     about: String,
///     version: String,
///     author: String,
///     command_map: HashMap<String, DisplayCommand>,
/// }
///
/// impl ParserInfo {
///     pub fn new() -> Self {
///         ParserInfo {
///             command_map: HashMap::new(),
///         }
///     }
/// }
/// ````
#[proc_macro_attribute]
pub fn cli(attribute: TokenStream, item: TokenStream) -> TokenStream {
    let parser = RustParser::new();
    // the item is always on a parser struct.
    let ast = parser.parse_struct(&item);
    cli_parser::parse(attribute, ast, item)
}

#[proc_macro_attribute]
pub fn help(attribute: TokenStream, item: TokenStream) -> TokenStream {
    help::parse(attribute, item)
}
