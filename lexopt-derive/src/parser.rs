use kproc_parser::kparser::{KParserError, KParserTracer};
use kproc_parser::proc_macro::TokenStream;
use kproc_parser::rust::ast_nodes::{FieldToken, TopLevelNode};
use kproc_parser::rust::kparser::RustParser;
use kproc_parser::{trace, build_error};
use proc_macro::TokenTree;

use crate::Tracer;

pub struct ParserInfo {
    pub subcommands: Vec<SubCommandInfo>,
    pub flags: Vec<ArgsInfo>,
    pub custom_help: bool,
    pub custom_parse: bool,
}

pub struct SubCommandInfo {
    pub name: TokenTree,
}

pub struct ArgsInfo {
    pub long_name: TokenTree,
    pub short_name: TokenTree,
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
    let Ok(ast) = parser.parse(&stream) else {
        unreachable!()
    };
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
    ast: TopLevelNode,
    tracer: &T,
) -> Result<ParserInfo, KParserError> {
    let mut info = ParserInfo {
        subcommands: vec![],
        flags: vec![],
        custom_help: false,
        custom_parse: false,
    };
    match ast {
        TopLevelNode::Struct(ast) => {
            for field in ast.fields {
                trace!(tracer, "{field}");
                if field.attrs.contains_key("subcommand") {
                    info.subcommands.push(SubCommandInfo {
                        name: field.identifier,
                    });
                } else {
                    // FIXME: we should be able to rename the fields
                    info.flags.push(ArgsInfo {
                        long_name: field.identifier.clone(),
                        short_name: field.identifier,
                    });
                }
            }
        }
        TopLevelNode::Enum(ast) => {
            for value in ast.values {
                info.subcommands.push(SubCommandInfo {
                    name: value.identifier,
                })
            }
        }
        _ => unimplemented!(),
    }
    Ok(info)
}
