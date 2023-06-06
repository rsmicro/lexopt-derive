use kproc_parser::kparser::{KParserError, KParserTracer};
use kproc_parser::proc_macro::TokenStream;
use kproc_parser::rust::ast_nodes::{FieldToken, StructToken};
use kproc_parser::rust::kparser::RustParser;
use kproc_parser::trace;

use crate::Tracer;

pub struct ParserInfo {
    pub subcommands: Vec<FieldToken>,
    pub flags: Vec<FieldToken>,
    pub custom_help: bool,
    pub custom_parse: bool,
}

impl ParserInfo {
    pub fn to_tokens_stream(self) -> TokenStream {
        self.to_string().parse().unwrap()
    }
}

impl std::fmt::Display for ParserInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

// FIXME: pass down the struct attribute
pub fn parse(stream: TokenStream) -> TokenStream {
    let tracer = Tracer {};
    let parser = RustParser::with_tracer(&tracer);
    let ast = parser.parse_struct(&stream);
    let parser_impl = generate_parser(ast, &tracer);
    if let Err(err) = parser_impl {
        err.emit();
        panic!();
    }
    // SAFETY: it is safe to unwrap because
    // we check and mange the error before.
    parser_impl.unwrap().to_tokens_stream()
}

pub fn generate_parser<T: KParserTracer>(
    ast: StructToken,
    tracer: &T,
) -> Result<ParserInfo, KParserError> {
    let mut info = ParserInfo {
        subcommands: vec![],
        flags: vec![],
        custom_help: false,
        custom_parse: false,
    };
    for field in ast.fields {
        trace!(tracer, "{field}");
        if field.attrs.contains_key("subcommand") {
            info.subcommands.push(field);
        } else {
            info.subcommands.push(field);
        }
    }
    Ok(info)
}
