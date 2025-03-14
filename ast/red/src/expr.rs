use std::fmt;
use std::ops::Deref;

use derive_more::{From, Into};
use sappho_ast_effect::Effect;
use sappho_ast_kernel::{
    ApplicationExpr, Interaction, Kernel, LetExpr, LookupExpr, MatchExpr, ObjectDef,
};
use sappho_attrs::Attrs;
use sappho_identifier::RcId;
use sappho_unparse::{Stream, Unparse};

use crate::AstRed;

#[derive(Clone, Debug, PartialEq, From, Into)]
#[from(
    Kernel<AstRed, FX>,
    RcId,
    f64,
    ObjectDef<AstRed, FX>,
    LetExpr<AstRed, FX>,
    MatchExpr<AstRed, FX>,
    ApplicationExpr<AstRed, FX>,
    LookupExpr<AstRed, FX>,
    Interaction<AstRed, FX>,


)]
pub struct Expr<FX>(Kernel<AstRed, FX>)
where
    FX: Effect;

impl<FX> Expr<FX>
where
    FX: Effect,
{
    pub fn new<T>(x: T) -> Self
    where
        Kernel<AstRed, FX>: From<T>,
    {
        Expr(Kernel::from(x))
    }
}

impl<FX> From<Attrs<Expr<FX>>> for Expr<FX>
where
    FX: Effect,
{
    fn from(attrs: Attrs<Expr<FX>>) -> Self {
        Expr(Kernel::from(attrs))
    }
}

impl<FX> Deref for Expr<FX>
where
    FX: Effect,
{
    type Target = Kernel<AstRed, FX>;

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
