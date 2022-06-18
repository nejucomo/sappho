use crate::{AstFxFor, FromFx, ObjectDef, RecursiveExpr, UniversalExpr};
use sappho_ast as ast;
use std::fmt;

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

impl<FX> fmt::Display for GenExpr<FX>
where
    FX: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GenExpr::*;

        match self {
            Universal(x) => x.fmt(f),
            Common(x) => x.fmt(f),
            Recursive(x) => x.fmt(f),
            Effect(x) => x.fmt(f),
        }
    }
}
