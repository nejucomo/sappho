use derive_new::new;
use sappho_ast_effect::Effect;
use sappho_unparse::Unparse;

use crate::{AstProvider, CommentedExpr};

#[derive(Clone, Debug, PartialEq, new)]
pub struct BoxExpr<XP, FX>(Box<CommentedExpr<XP, FX>>)
where
    XP: AstProvider,
    FX: Effect;

impl<XP, FX> BoxExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    pub fn into_inner(self) -> CommentedExpr<XP, FX> {
        *(self.0)
    }

    pub fn map_expr<F, XPD, FXD>(self, f: F) -> BoxExpr<XPD, FXD>
    where
        F: FnOnce(XP::Expr<FX>) -> XPD::Expr<FXD>,
        XPD: AstProvider,
        FXD: Effect,
    {
        BoxExpr::from(self.0.map_expr(f))
    }

    pub fn try_map_expr<F, XPD, FXD, E>(self, f: F) -> Result<BoxExpr<XPD, FXD>, E>
    where
        F: FnOnce(XP::Expr<FX>) -> Result<XPD::Expr<FXD>, E>,
        XPD: AstProvider,
        FXD: Effect,
    {
        self.0.try_map_expr(f).map(BoxExpr::from)
    }
}

impl<XP, FX> From<CommentedExpr<XP, FX>> for BoxExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn from(value: CommentedExpr<XP, FX>) -> Self {
        Self(Box::new(value))
    }
}

impl<XP, FX> Unparse for BoxExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn unparse_into(&self, s: &mut sappho_unparse::Stream) {
        self.0.unparse_into(s)
    }
}
