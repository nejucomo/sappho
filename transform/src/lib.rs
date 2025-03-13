mod xform;

use sappho_ast as ast;
use sappho_ast_reduced as astred;

use crate::xform::TransformInto;

pub fn reduce(expr: ast::PureExpr) -> astred::PureExpr {
    expr.transform()
}

pub fn canonicalize(expr: ast::PureExpr) -> ast::PureExpr {
    let redx: astred::PureExpr = expr.transform();
    redx.transform()
}

#[cfg(test)]
mod tests;
