use derive_new::new;
use sappho_ast_comments::Commented;
use sappho_ast_effect::Effect;
use sappho_unparse::Unparse;

use crate::ExprProvider;

#[derive(Clone, Debug, PartialEq, new)]
pub struct CommentedExpr<XP, FX>(Commented<XP::Expr<FX>>)
where
    XP: ExprProvider,
    FX: Effect;

impl<XP, FX> CommentedExpr<XP, FX>
where
    XP: ExprProvider,
    FX: Effect,
{
    pub fn new_bare<X>(expr: X) -> Self
    where
        XP::Expr<FX>: From<X>,
    {
        CommentedExpr::new(Commented::from(XP::Expr::from(expr)))
    }

    pub fn map_expr<F, XPD>(self, f: F) -> CommentedExpr<XPD, FX>
    where
        F: FnOnce(XP::Expr<FX>) -> XPD::Expr<FX>,
        XPD: ExprProvider,
    {
        CommentedExpr::new(self.0.map(f))
    }
}

impl<XP, FX> Unparse for CommentedExpr<XP, FX>
where
    XP: ExprProvider,
    FX: Effect,
{
    fn unparse_into(&self, s: &mut sappho_unparse::Stream) {
        self.0.unparse_into(s)
    }
}
