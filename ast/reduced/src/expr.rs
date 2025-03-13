use std::fmt;
use std::ops::Deref;

use derive_more::{From, Into};
use sappho_ast_core::CoreExpr;
use sappho_ast_effect::Effect;
use sappho_attrs::Attrs;
use sappho_unparse::{Stream, Unparse};

use crate::AstRed;

#[derive(Clone, Debug, PartialEq, From, Into)]
pub struct Expr<FX>(CoreExpr<AstRed, FX>)
where
    FX: Effect;

impl<FX> Expr<FX>
where
    FX: Effect,
{
    pub fn new<T>(x: T) -> Self
    where
        CoreExpr<AstRed, FX>: From<T>,
    {
        Expr(CoreExpr::from(x))
    }
}

impl<FX> From<Attrs<Expr<FX>>> for Expr<FX>
where
    FX: Effect,
{
    fn from(attrs: Attrs<Expr<FX>>) -> Self {
        Expr(CoreExpr::from(attrs))
    }
}

impl<FX> Deref for Expr<FX>
where
    FX: Effect,
{
    type Target = CoreExpr<AstRed, FX>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<FX> Unparse for Expr<FX>
where
    FX: Effect,
{
    fn unparse_into(&self, s: &mut Stream) {
        self.0.unparse_into(s);
    }
}

impl<FX> fmt::Display for Expr<FX>
where
    FX: Effect,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unparse().fmt(f)
    }
}
