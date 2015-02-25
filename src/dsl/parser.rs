use syntax::{ast, codemap};
use syntax::ext::base;
use syntax::parse::parser;

impl<'a, 'b> super::super::Parser<(codemap::Span, &'a mut base::ExtCtxt<'b>)> for super::DslState {
    fn parse(parser: &mut parser::Parser, (_sp, _cx): (codemap::Span, &'a mut base::ExtCtxt))
        -> super::DslState
    {
        let expr = parser.parse_expr();
        match &expr.node {
            &ast::Expr_::ExprClosure(..) => (),
            _ => panic!("Only closure expressions are allowed")
        };

        super::DslState{
            expr: expr,
        }
    }
}
