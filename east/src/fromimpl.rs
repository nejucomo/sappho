use crate::{Application, Expr, FuncClause, LetExpr, ObjectExpr};
use saplang_ast as ast;

impl From<ast::Expr> for Expr {
    fn from(ae: ast::Expr) -> Expr {
        use Expr::*;

        match ae {
            ast::Expr::Lit(x) => Lit(x),
            ast::Expr::Ref(x) => Ref(x),
            ast::Expr::List(x) => List(x.into_iter().map(Expr::from).collect()),
            ast::Expr::Let(x) => Let(x.into()),
            ast::Expr::Func(x) => Object(x.into()),
            ast::Expr::Apply(x) => Apply(x.into()),
            ast::Expr::Object(x) => Object(x.into()),
        }
    }
}

impl From<ast::LetExpr> for LetExpr {
    fn from(ale: ast::LetExpr) -> LetExpr {
        LetExpr {
            binding: ale.binding,
            bindexpr: box_expr_from(ale.bindexpr),
            tail: box_expr_from(ale.tail),
        }
    }
}

impl From<ast::Application> for Application {
    fn from(aa: ast::Application) -> Application {
        Application {
            target: box_expr_from(aa.target),
            argument: box_expr_from(aa.argument),
        }
    }
}

impl From<ast::ObjectExpr> for ObjectExpr {
    fn from(ao: ast::ObjectExpr) -> ObjectExpr {
        ObjectExpr {
            func: ao.func.map(FuncClause::from),
        }
    }
}

impl From<ast::FuncExpr> for ObjectExpr {
    fn from(fe: ast::FuncExpr) -> ObjectExpr {
        ObjectExpr {
            func: Some(FuncClause::from(fe)),
        }
    }
}

impl From<ast::FuncExpr> for FuncClause {
    fn from(fe: ast::FuncExpr) -> FuncClause {
        FuncClause {
            binding: fe.binding,
            body: std::rc::Rc::new(Expr::from(*fe.body)),
        }
    }
}

#[allow(clippy::boxed_local)]
fn box_expr_from(b: Box<ast::Expr>) -> Box<Expr> {
    Box::new(Expr::from(*b))
}
