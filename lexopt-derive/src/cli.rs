use kproc_parser::kparser::{KParserError, KParserTracer};
use kproc_parser::kproc_macros::KTokenStream;
use kproc_parser::proc_macro::{TokenStream, TokenTree};
use kproc_parser::rust::ast_nodes::StructToken;

use kproc_parser::{build_error, check};

use crate::Tracer;

pub struct CliHelper {
    pub name: TokenTree,
    pub about: TokenTree,
    pub version: Option<TokenTree>,
    pub author: Option<TokenTree>,
    pub on_ty: Option<TokenTree>,
    pub item: TokenStream,
}

impl CliHelper {
    pub fn to_token_stream(self) -> TokenStream {
        self.to_string().parse().unwrap()
    }

    fn parse(
        stream: &mut KTokenStream,
        _: &dyn KParserTracer,
        ast: &StructToken,
        item: TokenStream,
    ) -> Result<Self, KParserError> {
        let mut name: Option<TokenTree> = Some(ast.name.clone());
        let mut about: Option<TokenTree> = None;
        let mut version: Option<TokenTree> = None;
        let mut author: Option<TokenTree> = None;

        let mut last_token: Option<TokenTree> = None;
        while !stream.is_end() {
            let key = stream.advance();
            check!("=", stream.peek())?;
            let _ = stream.advance();
            let value = stream.advance();
            if !stream.is_end() && stream.match_tok(",") {
                check!(",", stream.advance())?;
            }

            match key.to_string().as_str() {
                "name" => name = Some(value),
                "about" => about = Some(value),
                "version" => version = Some(value),
                "author" => author = Some(value),
                _ => return Err(build_error!(key, "cli value not found")),
            };
            last_token = Some(key);
        }
        Ok(Self {
            name: name.ok_or(build_error!(
                last_token.clone().unwrap(),
                "name must be specified"
            ))?,
            about: about.ok_or(build_error!(
                last_token.clone().unwrap(),
                "about must be specified"
            ))?,
            version,
            author,
            on_ty: None,
            item,
        })
    }
}

impl std::fmt::Display for CliHelper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.item)
    }
}

pub fn parse(tokens: TokenStream, struct_ast: StructToken, raw_item: TokenStream) -> TokenStream {
    let tracer = Tracer {};
    let mut stream = KTokenStream::new(&tokens);
    let info = CliHelper::parse(&mut stream, &tracer, &struct_ast, raw_item);
    if let Err(err) = info {
        err.emit();
        panic!()
    }
    let mut info = info.unwrap();
    info.on_ty = Some(struct_ast.name);
    info.to_token_stream()
}
