use kproc_parser::build_error;
use kproc_parser::kparser::KParserError;
use kproc_parser::kparser::KParserTracer;
use kproc_parser::kproc_macros::KTokenStream;
use kproc_parser::proc_macro::TokenStream;
use kproc_parser::rust::ast_nodes::Bound;
use kproc_parser::rust::ast_nodes::GenericParams;
use kproc_parser::rust::ast_nodes::{FnDeclTok, TyToken};
use kproc_parser::rust::kparser::RustParser;
use kproc_parser::trace;
use proc_macro::TokenTree;

use crate::Tracer;

pub struct HelpParsing {
    pub on_ty: TokenTree,
    pub fn_params: TokenStream,
    pub body: TokenStream,
    pub bounds: Option<GenericParams>,
}

impl HelpParsing {
    pub fn to_tokenstream(self) -> TokenStream {
        self.to_string().parse().unwrap()
    }
}

impl std::fmt::Display for HelpParsing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ty = self.on_ty.clone();
        let body = self.body.clone();
        let fn_params = self.fn_params.clone();
        let generics_params = self
            .bounds
            .clone()
            .and_then(|params| Some(params.to_string()))
            .unwrap_or("".to_owned());
        writeln!(f, "pub fn help{generics_params}({fn_params}) {{")?;
        writeln!(f, "{body}")?;
        writeln!(f, "}}")
    }
}

pub fn parse(tokens: TokenStream, item: TokenStream) -> TokenStream {
    let tracer = Tracer {};
    let parser = RustParser::with_tracer(&tracer);

    let mut attr_tokens = KTokenStream::new(&tokens);
    let on_ty = attr_tokens.advance();

    let fn_ast = parser.parse_fn(&item);
    let result = parse_help_fn(tokens, on_ty, fn_ast);
    if let Err(err) = result {
        err.emit();
        panic!();
    }
    trace!(
        tracer,
        "parse done with success: {}",
        result.as_ref().unwrap()
    );
    result.unwrap().to_tokenstream()
}

fn parse_help_fn(
    tokens: TokenStream,
    on_type: TokenTree,
    function_ast: FnDeclTok,
) -> Result<HelpParsing, KParserError> {
    let help = HelpParsing {
        on_ty: on_type,
        fn_params: function_ast.raw_params,
        bounds: function_ast.generics,
        body: function_ast.raw_body.ok_or(build_error!(
            function_ast.ident.clone(),
            "missing function body"
        ))?,
    };
    Ok(help)
}
