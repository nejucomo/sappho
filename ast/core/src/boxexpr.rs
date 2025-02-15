use derive_new::new;
use sappho_ast_effect::Effect;
use sappho_unparse::Unparse;

use crate::{AstProvider, AstTransformInto, CommentedExpr};

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
        BoxExpr::new(Box::new(value))
    }
}

impl<XPD, XPS, FX> AstTransformInto<BoxExpr<XPD, FX>> for BoxExpr<XPS, FX>
where
    XPD: AstProvider,
    XPS: AstProvider,
    XPS::Expr<FX>: AstTransformInto<XPD::Expr<FX>>,
    FX: Effect,
{
    fn ast_transform(self) -> BoxExpr<XPD, FX> {
        BoxExpr(Box::new(self.into_inner().ast_transform()))
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
