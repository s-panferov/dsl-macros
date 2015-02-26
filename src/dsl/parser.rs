use syntax::{ast, codemap};
use syntax::ext::base;
use syntax::parse::parser;

impl<'a, 'b> super::super::Parser<(codemap::Span, &'a mut base::ExtCtxt<'b>)> for super::DslState {
    fn parse(parser: &mut parser::Parser, (sp, cx): (codemap::Span, &'a mut base::ExtCtxt))
        -> super::DslState
    {
        let expr = parser.parse_expr();
        match &expr.node {
            &ast::Expr_::ExprClosure(..) => (),
            _ => cx.span_fatal(sp, "Only closure expressions are allowed inside dsl!()")
        };

        super::DslState{
            expr: expr,
        }
    }
}
