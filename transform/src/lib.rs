use sappho_ast as ast;
use sappho_ast_reduced as astred;

pub fn reduce(expr: ast::PureExpr) -> astred::PureExpr {
    astred::PureExpr::from(expr)
}

pub fn canonicalize(expr: ast::PureExpr) -> ast::PureExpr {
    ast::PureExpr::from(astred::PureExpr::from(expr))
}
