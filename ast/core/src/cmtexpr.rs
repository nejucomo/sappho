use derive_new::new;
use sappho_ast_comments::Commented;
use sappho_ast_effect::Effect;
use sappho_identmap::{IdentMap, TryIntoIdentMap};
use sappho_unparse::Unparse;

use crate::{AstProvider, AstTransformInto};

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

    pub fn map_expr<F, XPD>(self, f: F) -> CommentedExpr<XPD, FX>
    where
        F: FnOnce(XP::Expr<FX>) -> XPD::Expr<FX>,
        XPD: AstProvider,
    {
        CommentedExpr::new(self.0.map(f))
    }
}

impl<XPD, XPS, FX> AstTransformInto<CommentedExpr<XPD, FX>> for CommentedExpr<XPS, FX>
where
    XPD: AstProvider,
    XPS: AstProvider,
    XPS::Expr<FX>: AstTransformInto<XPD::Expr<FX>>,
    FX: Effect,
{
    fn ast_transform(self) -> CommentedExpr<XPD, FX> {
        CommentedExpr(self.0.ast_transform())
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
