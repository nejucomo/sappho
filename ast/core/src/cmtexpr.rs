use derive_new::new;
use sappho_ast_comments::Commented;
use sappho_ast_effect::Effect;
use sappho_identmap::{IdentMap, TryIntoIdentMap};
use sappho_unparse::Unparse;

use crate::AstProvider;

#[derive(Clone, Debug, PartialEq, new)]
pub struct CommentedExpr<XP, FX>(Commented<XP::Expr<FX>>)
where
    XP: AstProvider,
    FX: Effect;

impl<XP, FX> CommentedExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    pub fn new_bare<X>(expr: X) -> Self
    where
        XP::Expr<FX>: From<X>,
    {
        CommentedExpr::new(Commented::from(XP::Expr::from(expr)))
    }

    pub fn into_inner(self) -> (String, XP::Expr<FX>) {
        self.0.into_inner()
    }

    pub fn map_expr<F, XPD, FXD>(self, f: F) -> CommentedExpr<XPD, FXD>
    where
        F: FnOnce(XP::Expr<FX>) -> XPD::Expr<FXD>,
        XPD: AstProvider,
        FXD: Effect,
    {
        CommentedExpr::new(self.0.map(f))
    }

    pub fn try_map_expr<F, XPD, FXD, E>(self, f: F) -> Result<CommentedExpr<XPD, FXD>, E>
    where
        F: FnOnce(XP::Expr<FX>) -> Result<XPD::Expr<FXD>, E>,
        XPD: AstProvider,
        FXD: Effect,
    {
        self.0.try_map(f).map(CommentedExpr::new)
    }
}

impl<XP, FX> Unparse for CommentedExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn unparse_into(&self, s: &mut sappho_unparse::Stream) {
        self.0.unparse_into(s)
    }
}

impl<XP, FX> TryIntoIdentMap<CommentedExpr<XP, FX>> for CommentedExpr<XP, FX>
where
    XP: AstProvider,
    XP::Expr<FX>: TryIntoIdentMap<CommentedExpr<XP, FX>>,
    FX: Effect,
{
    fn try_into_identmap(&self) -> Option<&IdentMap<CommentedExpr<XP, FX>>> {
        self.0.item().try_into_identmap()
    }
}
