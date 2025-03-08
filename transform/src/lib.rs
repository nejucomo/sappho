mod xform;

use sappho_ast_reduced as red;
use sappho_ast_rich as rich;

use crate::xform::TransformInto;

pub fn reduce(expr: rich::PureExpr) -> red::PureExpr {
    expr.transform()
}

pub fn canonicalize(expr: rich::PureExpr) -> rich::PureExpr {
    let redx: red::PureExpr = expr.transform();
    redx.transform()
}

#[cfg(test)]
mod tests;
