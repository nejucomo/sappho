use sappho_ast as ast;
use sappho_east as east;

pub fn reduce(expr: ast::PureExpr) -> east::PureExpr {
    east::PureExpr::from(expr)
}

pub fn canonicalize(expr: ast::PureExpr) -> ast::PureExpr {
    ast::PureExpr::from(east::PureExpr::from(expr))
}
