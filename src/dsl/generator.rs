use syntax::{codemap, ast};
use syntax::ptr;
use syntax::ext::base;
use syntax::ext::build::AstBuilder;

fn rewrite(expr: ast::Expr, context: ast::Ident, cx: &mut base::ExtCtxt) -> ptr::P<ast::Expr> {
    let ast::Expr{span, node, ..} = expr;
    match node {
        ast::Expr_::ExprCall(path, mut args) => {
            let (ident, types) = match (*path).clone().node {
                ast::Expr_::ExprPath(_, path) => {
                    let ast::PathSegment{identifier, parameters} = path.segments[0].clone();
                    let types = match parameters {
                        ast::PathParameters::AngleBracketedParameters(data) => {
                            data.types
                        },
                        _ => unreachable!()
                    };
                    (identifier, types.into_vec())
                },
                _ => unreachable!()
            };

            args.insert(0, cx.expr_path(cx.path_ident(span, context)));
            cx.expr(span, ast::ExprMethodCall(codemap::Spanned{node: ident, span: span}, types, args))
        },
        _ => unreachable!()
    }
}

fn rewrite_if_needed(expr: &mut ptr::P<ast::Expr>, context: ast::Ident, cx: &mut base::ExtCtxt) {
    let is_call = match &(*expr).node {
        &ast::Expr_::ExprCall(ref path, _) => {
            match path.node {
                ast::Expr_::ExprPath(_, ref path) if path.segments.len() == 1 => true,
                _ => false
            }
        },
        _ => false
    };
    if is_call {
        *expr = rewrite((**expr).clone(), context, cx);
    }
}

fn extract_name(arg: ast::Arg, sp: codemap::Span, cx: &mut base::ExtCtxt) -> ast::Ident {
    match arg.pat.node {
        ast::Pat_::PatIdent(_, ident, _) => {
            ident.node.clone()
        },
        _ => cx.span_fatal(sp, "Only ident is possible as a first argument inside dsl!()")
    }
}

impl super::super::Generator<()> for super::DslState {
    fn generate<'a>(mut self, sp: codemap::Span, cx: &mut base::ExtCtxt, _: ()) -> Box<base::MacResult + 'a> {
        self.expr = self.expr.map(|mut expr| {
            match &mut expr.node {
                &mut ast::Expr_::ExprClosure(ref _clause, ref decl, ref mut block) => {
                    let first_arg = extract_name(if decl.inputs.len() == 0 {
                        cx.span_fatal(sp, "The closure expression must have at least one argument inside dsl!()")
                    } else {
                        decl.inputs[0].clone()
                    }, sp, cx);

                    *block = block.clone().map(|mut block| {
                        block.expr = Some(block.expr.unwrap().clone().map(|mut expr| {
                            match &mut expr.node {
                                &mut ast::Expr_::ExprBlock(ref mut block) => {
                                    *block = block.clone().map(|mut block| {
                                        if block.expr.is_some() {
                                            rewrite_if_needed(block.expr.as_mut().unwrap(), first_arg, cx);
                                        }
                                        for stmt in block.stmts.iter_mut() {
                                            *stmt = stmt.clone().map(|mut stmt| {
                                                match &mut stmt.node {
                                                    &mut ast::Stmt_::StmtDecl(ref _decl, ref _id) => (),
                                                    &mut ast::Stmt_::StmtMac(ref _mac, ref _style) => (),
                                                    &mut ast::Stmt_::StmtExpr(ref mut expr, ref _id) => {
                                                        rewrite_if_needed(expr, first_arg, cx);
                                                    }
                                                    &mut ast::Stmt_::StmtSemi(ref mut expr, ref _id) => {
                                                        rewrite_if_needed(expr, first_arg, cx);
                                                    }
                                                }
                                                stmt
                                            });
                                        }

                                        block
                                    })
                                },
                                _ => ()
                            }
                            expr
                        }));
                        block
                    });
                },
                _ => unreachable!()
            };

            expr
        });

        base::MacEager::expr(self.expr)
    }
}

