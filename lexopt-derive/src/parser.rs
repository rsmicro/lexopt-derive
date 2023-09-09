use kproc_parser::kparser::{KParserError, KParserTracer};
use kproc_parser::proc_macro::TokenStream;
use kproc_parser::proc_macro::TokenTree;
use kproc_parser::rust::ast_nodes::TopLevelNode;
use kproc_parser::rust::kparser::RustParser;
use kproc_parser::trace;

use crate::TRACER;

macro_rules! build_parser {
    ($tracer:expr) => {
        RustParser::with_tracer(&$tracer)
    };
}

pub struct ParserMacroInfo {
    pub identifier: Option<TokenTree>,
    pub subcommands: Vec<SubCommandInfo>,
    pub flags: Vec<ArgsInfo>,
    pub custom_help: bool,
    pub custom_parse: bool,
}

pub struct SubCommandInfo {
    pub name: TokenTree,
    pub ty: TokenTree,
}

pub struct ArgsInfo {
    pub ty: TokenTree,
    pub long_name: TokenTree,
    pub short_name: TokenTree,
}

impl ParserMacroInfo {
    pub fn to_tokens_stream(self) -> TokenStream {
        self.to_string().parse().unwrap()
    }
}

impl std::fmt::Display for ParserMacroInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut code = r#"
        use lexopt_helper::prelude::*;
"#
        .to_string();

        let struct_identifier = self.identifier.clone().unwrap();
        code += &format!("\nimpl {struct_identifier} {{");

        let mut declarations = String::new();
        for subcommands in self.subcommands.iter() {
            let identifier = subcommands.name.clone();
            let ty = subcommands.ty.clone();
            declarations += &format!("let mut {identifier}: Option<{ty}> = None;\n");
        }
        for flags in self.flags.iter() {}

        code += &format!(
            "fn new() -> Self {{
                    unimplemented!()
}}\n"
        );

        code += &format!(
            "fn parse() -> Result<Self, Error> {{
                            let mut parser = ParserInfo::new();
                            {declarations}

                            while let Some(arg) = parser.next()? {{
                                println!(\"{{:?}}\", arg);
                            }}

                            Ok(Self::new())
                          }}"
        );
        code += "}"; // close the impl
        writeln!(f, "{code}")
    }
}

/// EXPAND: Procedural macros expands result in the Parser implementation
/// with all the meta information that the parser needs a runtime.
///
/// ```norun
/// use lexopt_helper::prelude::*;
/// ````
pub fn parse(stream: TokenStream) -> TokenStream {
    let parser = build_parser!(TRACER);
    let ast = parser.parse(&stream).unwrap();
    let Ok(parser_impl) = generate_parser(ast, &TRACER).map_err(|err| err.emit()) else {
        unimplemented!()
    };
    parser_impl.to_tokens_stream()
}

pub fn generate_parser<T: KParserTracer>(
    ast: TopLevelNode,
    tracer: &T,
) -> Result<ParserMacroInfo, KParserError> {
    let mut info = ParserMacroInfo {
        identifier: None,
        subcommands: vec![],
        flags: vec![],
        custom_help: false,
        custom_parse: false,
    };
    match ast {
        TopLevelNode::Struct(ast) => {
            info.identifier = Some(ast.name);
            for field in ast.fields {
                trace!(tracer, "{field}");
                if field.attrs.contains_key("subcommand") {
                    info.subcommands.push(SubCommandInfo {
                        name: field.identifier,
                        // FIXME: the type is more complex we are missing
                        // the generics.
                        ty: field.ty.identifier,
                    });
                } else {
                    // FIXME: we should be able to rename the fields
                    info.flags.push(ArgsInfo {
                        // FIXME: the ty is more complex, we are missing the
                        // generics
                        ty: field.ty.identifier,
                        long_name: field.identifier.clone(),
                        short_name: field.identifier,
                    });
                }
            }
        }
        TopLevelNode::Enum(ast) => {
            info.identifier = Some(ast.identifier.clone());
            for value in ast.values {
                info.subcommands.push(SubCommandInfo {
                    name: value.identifier,
                    ty: ast.identifier.clone(),
                })
            }
        }
        _ => unimplemented!(),
    }
    Ok(info)
}
