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

    pub fn map_expr<F, XPD>(self, f: F) -> BoxExpr<XPD, FX>
    where
        F: FnOnce(XP::Expr<FX>) -> XPD::Expr<FX>,
        XPD: AstProvider,
    {
        BoxExpr(Box::new((*self.0).map_expr(f)))
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
