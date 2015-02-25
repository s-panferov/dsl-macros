use syntax::{ast, ptr, codemap, parse};
use syntax::ext::base;

use super::Generator;

mod parser;
mod generator;

#[derive(Clone)]
pub struct DslState {
    pub expr: ptr::P<ast::Expr>,
}

pub fn dsl<'cx>(cx: &'cx mut base::ExtCtxt, sp: codemap::Span, tokens: &[ast::TokenTree])
    -> Box<base::MacResult + 'cx>
{
    // Parse a full ModelState from the input, emitting errors if used incorrectly.
    let mut parser = parse::tts_to_parser(cx.parse_sess(), tokens.to_vec(), cx.cfg());
    let state: DslState = super::Parser::parse(&mut parser, (sp, &mut*cx));
    state.generate(sp, cx, ())
}