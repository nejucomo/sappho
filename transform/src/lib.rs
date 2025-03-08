mod xform;

use sappho_ast_reduced as astred;
use sappho_ast_rich as ast;

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
