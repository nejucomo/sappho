mod xform;

use sappho_ast_red as astred;
use sappho_ast_rich as rich;

use crate::xform::TransformInto;

pub fn reduce(expr: rich::PureExpr) -> astred::PureExpr {
    expr.transform()
}

pub fn canonicalize(expr: rich::PureExpr) -> rich::PureExpr {
    let redx: astred::PureExpr = expr.transform();
    redx.transform()
}

#[cfg(test)]
mod tests;
