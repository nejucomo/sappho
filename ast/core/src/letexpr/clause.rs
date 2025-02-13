use sappho_ast_effect::Effect;
use sappho_unparse::{Stream, Unparse};

use crate::{AstProvider, AstTransformInto, BoxExpr};

#[derive(Debug, derive_new::new)]
pub struct LetClause<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    /// The binding pattern, ie: the first `x` in `let x = 42; f x`.
    pub binding: XP::Pattern,

    /// The expression to bind, ie: `42` in `let x = 42; f x`.
    pub bindexpr: BoxExpr<XP, FX>,
}

impl<XPD, XPS, FX> AstTransformInto<LetClause<XPD, FX>> for LetClause<XPS, FX>
where
    XPD: AstProvider,
    XPS: AstProvider,
    XPS::Pattern: AstTransformInto<XPD::Pattern>,
    XPS::Expr<FX>: AstTransformInto<XPD::Expr<FX>>,
    FX: Effect,
{
    fn ast_transform(self) -> LetClause<XPD, FX> {
        LetClause {
            binding: self.binding.ast_transform(),
            bindexpr: self.bindexpr.ast_transform(),
        }
    }
}

impl<XP, FX> Unparse for LetClause<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn unparse_into(&self, s: &mut Stream) {
        s.write("let ");
        s.write(&self.binding);
        s.write(" = ");
        s.write(&self.bindexpr);
    }
}

impl<XP, FX> Clone for LetClause<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn clone(&self) -> Self {
        LetClause::new(self.binding.clone(), self.bindexpr.clone())
    }
}

impl<XP, FX> PartialEq for LetClause<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn eq(&self, other: &Self) -> bool {
        self.binding == other.binding && self.bindexpr == other.bindexpr
    }
}
