//! Sub Command derive macro implementation
use std::fmt;

use kproc_parser::kparser::{KParserError, KParserTracer};
use kproc_parser::proc_macro::{TokenStream, TokenTree};
use kproc_parser::rust::ast_nodes::TopLevelNode;
use kproc_parser::rust::kenum::EnumValueKind;

use crate::macros::build_parser;
use crate::parser::{ArgsInfo, SubCommandInfo};
use crate::TRACER;

struct SubCommandMacroInfo {
    pub identifier: Option<TokenTree>,
    pub subcommand: Vec<MacroInfo>,
}

struct MacroInfo {
    pub identifier: TokenTree,
    /// All the fields that are defined inside
    /// the subcommand.
    pub fields: Vec<ArgsInfo>,
    /// All the sub commands that a subcommand will
    /// implement
    pub subcommands: Vec<SubCommandInfo>,
}

// FIXME: we need to register the command inside the symbol table
//
//        parser.command_map.insert(
//            "install".to_owned(),
//            DisplayCommand {
//                name: "install".to_owned(),
//                subcommands: vec![],
//                args: vec![DisplayArg {
//                    long_name: "name".to_owned(),
//                    short_name: "n".to_owned(),
//                    optional: true,
//                    description: String::new(),
//                }],
//                usage: "Random usage".to_owned(),
//                description: "random description".to_owned(),
//            },
//        );
impl fmt::Display for SubCommandMacroInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let idetifier = self.identifier.clone().unwrap();
        let mut match_body = String::new();
        let mut subcommands_fn = String::new();
        let mut subcommands_names = String::new();
        for subcommand in self.subcommand.iter() {
            let subcommand_name = subcommand.identifier.to_string();
            let identifier = subcommand.identifier.to_string().to_lowercase();
            match_body += &format!("\"{identifier}\" => Self::parse_{identifier}(parser),\n");

            let mut while_match = String::new();
            let mut declarations = String::new();
            let mut new_params = String::new();
            let mut self_assign = String::new();
            let mut self_new_call = String::new();
            // FIXME: Parse the subcommands
            for flag in subcommand.fields.iter() {
                let identifier = flag.long_name.clone();
                let ty = flag.ty.clone();
                declarations += &format!("let mut {identifier}: Option<{ty}> = None;\n");
                new_params += &format!("{identifier}: {ty},");
                self_assign += &format!("{identifier}: {identifier},");
                self_new_call += &format!("{identifier}: {identifier}.unwrap_or_default(),");
                while_match += &format!(
                    "Long(\"{identifier}\") => {{
                        println!(\"match in the subcommand\");
                        let value: {ty} = parser.value()?.parse()?;
                        {identifier} = Some(value);
                    }}\n"
                );
                // if the short name is specified, add it to the match
                if let Some(ref short_name) = flag.short_name {
                    while_match = format!("Short(\"{short_name}\") | {while_match}");
                }
            }

            // TODO: this needs to be move in another function
            subcommands_fn += &format!("pub fn parse_{identifier}(parser: &mut ParserInfo) -> Result<Self, Error> {{
                                                {declarations}
                                                loop {{
                                                    let Some(ref arg) = parser.next()? else {{ break; }};
                                                    println!(\"{{:?}}\", arg);
                                                    match arg.clone() {{
                                                        {while_match}
                                                        _ => return Err(arg.clone().unexpected()),
                                                    }}
                                                }}

                                    println!(\"returning parsered subcommand\");
                               Ok(Self::{subcommand_name}{{ {self_new_call}  }})
                                        }}\n");
            subcommands_names += &format!("\"{identifier}\",");
        }
        let subcommands_names = subcommands_names
            .strip_suffix(",")
            .unwrap_or(&subcommands_fn);

        let code = format!("impl {idetifier} {{\n
                                    pub fn parse<T: Display + ?Sized>(parser: &mut ParserInfo, cmd_val: &T) -> Result<Self, Error> {{
                                            match cmd_val.to_string().as_str() {{
                                                {match_body}
                                                _ => unreachable!(),
                                            }}
                                    }}

                                    pub fn is_this_subcommad<T: Display + ?Sized>(arg: &T) -> bool {{
                                       [{subcommands_names}].contains(&arg.to_string().as_str())
                                    }}
                                    {subcommands_fn}
                        }}");
        writeln!(f, "{code}")
    }
}

/// EXPAND: Procedural macros expands result in the Parser implementation
/// with all the meta information that the parser needs a runtime.
///
/// ```norun
/// use lexopt_helper::prelude::*;
///
/// struct NameOfYourParser {
///     pub verbose: bool,
/// }
///
/// // This is generated by the `Parser` derive macros
/// impl NameOfYourParser {
///     fn new(verbose: bool) -> Self {
///         Self{ verbose: verbose }
///     }
///
///     fn parse() -> Result<Self, Error> {
///         let mut parser = ParserInfo::new();
///         let mut verbose: Option<bool> = None;
///
///         while let Some(arg) parser.next()? {
///             match arg.clone() {
///                 // perform the match of the arguments
///                 _ => return Err(arg.unexpected()),
///             }
///         }
///         Ok(Self::new(verbose))
///     }
/// }
/// ````
pub fn parse(stream: TokenStream) -> TokenStream {
    let parser = build_parser!(TRACER);
    let Ok(ast) = parser.parse(&stream) else {
        unreachable!()
    };
    let Ok(implementation) = generate_impl(ast, &TRACER).map_err(|err| err.emit()) else {
        return "".parse().unwrap();
    };
    implementation.to_string().parse().unwrap()
}

fn generate_impl<T: KParserTracer>(
    ast: TopLevelNode,
    _: &T,
) -> Result<SubCommandMacroInfo, KParserError> {
    let mut info = SubCommandMacroInfo {
        identifier: None,
        subcommand: Vec::new(),
    };
    match ast {
        TopLevelNode::Enum(ast) => {
            info.identifier = Some(ast.identifier);
            for value in ast.values {
                // FIXME: we can reuse the parser code that we use to generate the
                // parser derive macro?
                let mut subcommands = MacroInfo {
                    identifier: value.identifier,
                    fields: Vec::new(),
                    subcommands: Vec::new(),
                };
                match value.kind {
                    // FIXME: the value should be a vector of struct
                    // FIXME: we are missing the inner attributes
                    EnumValueKind::Named(value) => {
                        for (identifier, ty) in value.into_iter() {
                            subcommands.fields.push(ArgsInfo {
                                ty,
                                long_name: identifier,
                                short_name: None,
                            });
                        }
                    }
                    EnumValueKind::Anonymus(_) => {
                        // FIXME: generate warning, that this kind of function
                    }
                    EnumValueKind::Simple => {}
                }
                // TODO: Feel the subcommand and fields
                info.subcommand.push(subcommands);
            }
        }
        _ => unimplemented!("only enum are supported as subcommands"),
    }

    Ok(info)
}
