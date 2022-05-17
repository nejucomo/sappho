use crate::{AstFxFor, FromFx, ObjectDef, RecursiveExpr, UniversalExpr};
use saplang_ast as ast;

#[derive(Debug, PartialEq)]
pub enum GenExpr<Effects> {
    Universal(UniversalExpr),
    Common(ObjectDef),
    Recursive(RecursiveExpr<Effects>),
    Effect(Effects),
}

impl<FX: FromFx> From<ast::GenExpr<AstFxFor<FX>>> for GenExpr<FX> {
    fn from(x: ast::GenExpr<AstFxFor<FX>>) -> Self {
        use GenExpr::*;

        match x {
            ast::GenExpr::Universal(x) => Universal(x),
            ast::GenExpr::Common(x) => Common(ObjectDef::from(x)),
            ast::GenExpr::Recursive(x) => Recursive(RecursiveExpr::from(x)),
            ast::GenExpr::Effect(x) => Effect(FX::from_fx(x)),
        }
    }
}
