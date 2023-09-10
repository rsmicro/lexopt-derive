use kproc_parser::kparser::{KParserError, KParserTracer};
use kproc_parser::proc_macro::TokenStream;
use kproc_parser::proc_macro::TokenTree;
use kproc_parser::rust::ast_nodes::TopLevelNode;
use kproc_parser::trace;

use crate::macros::build_parser;
use crate::TRACER;

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
    pub long_name: String,
    pub short_name: Option<TokenTree>,
}

impl ParserMacroInfo {
    pub fn to_tokens_stream(self) -> TokenStream {
        self.to_string().parse().unwrap()
    }
}

// FIXME: This needs to manage the commands info inside a symbol table
// An example of this?
//
//       parser.command_map.insert(
//            "@".to_owned(),
//            DisplayCommand {
//                name: "ex".to_owned(),
//                subcommands: vec![DisplayCommand {
//                    name: "install".to_owned(),
//                    subcommands: vec![],
//                    args: vec![DisplayArg {
//                        long_name: "name".to_owned(),
//                        short_name: "n".to_owned(),
//                        optional: true,
//                        description: String::new(),
//                    }],
//                    usage: "Random usage".to_owned(),
//                    description: "random description".to_owned(),
//                }],
//                args: vec![DisplayArg {
//                    description: "verbose".to_owned(),
//                    optional: true,
//                    long_name: "verbose".to_owned(),
//                    short_name: "v".to_owned(),
//                }],
//                usage: "ex [command] [--[options]]".to_owned(),
//                description: "a description".to_owned(),
//            },
//
//        );
impl std::fmt::Display for ParserMacroInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut code = r#"
        use lexopt_helper::prelude::*;
"#
        .to_string();

        let struct_identifier = self.identifier.clone().unwrap();
        code += &format!("\nimpl {struct_identifier} {{");

        let mut new_params = String::new();
        let mut self_assign = String::new();
        let mut self_new_call = String::new();
        let mut declarations = String::new();
        let mut while_match = String::new();
        for subcommands in self.subcommands.iter() {
            let identifier = subcommands.name.clone();
            let ty = subcommands.ty.clone();
            declarations += &format!("let mut {identifier}: Option<{ty}> = None;\n");
            new_params += &format!("{identifier}: {ty},");
            self_assign += &format!("{identifier}: {identifier},");
            // FIXME: there is a way to improve the unwrap or default?
            self_new_call += &format!("{identifier}.unwrap_or_default(),");
        }
        for flag in self.flags.iter() {
            let identifier = flag.long_name.clone();
            let ty = flag.ty.clone();
            declarations += &format!("let mut {identifier}: Option<{ty}> = None;\n");
            new_params += &format!("{identifier}: {ty},");
            self_assign += &format!("{identifier}: {identifier},");
            self_new_call += &format!("{identifier}.unwrap_or_default(),");
            while_match += &format!(
                "Long(\"{identifier}\") => {{
                    let value: {ty} = parser.value()?.parse()?;
                    {identifier} = Some(value);
                 }}\n"
            );
            // if the short name is specified, add it to the match
            if let Some(ref short_name) = flag.short_name {
                while_match = format!("Short(\"{short_name}\") | {while_match}");
            }
        }

        let new_params = new_params.strip_suffix(",").unwrap_or(&new_params);
        code += &format!(
            "fn new({new_params}) -> Self {{
                    Self{{ {self_assign} }}
            }}\n"
        );

        let self_new_call = self_new_call.strip_suffix(",").unwrap_or(&self_new_call);
        code += &format!(
            "fn parse() -> Result<Self, Error> {{
                            let mut parser = ParserInfo::new();
                            {declarations}

                            while let Some(arg) = parser.next()? {{
                                 println!(\"{{:?}}\",arg);
                                 match arg.clone() {{
                                      {while_match}
                                      _ => return Err(arg.unexpected()),
                                  }}
                            }}

                            Ok(Self::new({self_new_call}))
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
                        // FIXME: this should store the information as TokenTree
                        long_name: field.identifier.to_string(),
                        short_name: None,
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
