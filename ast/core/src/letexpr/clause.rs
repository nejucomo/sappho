use sappho_ast_effect::Effect;
use sappho_unparse::{Stream, Unparse};

use crate::AstProvider;

#[derive(Debug, derive_new::new)]
pub struct LetClause<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    /// The binding pattern, ie: the first `x` in `let x = 42; f x`.
    pub binding: XP::Pattern,

    /// The expression to bind, ie: `42` in `let x = 42; f x`.
    pub bindexpr: Box<XP::Expr<FX>>,
}

impl<XP, FX> LetClause<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    pub fn transform_into<XPD>(self) -> LetClause<XPD, FX>
    where
        XPD: AstProvider,
        XPD::Pattern: From<XP::Pattern>,
        XPD::Expr<FX>: From<XP::Expr<FX>>,
    {
        LetClause {
            binding: XPD::Pattern::from(self.binding),
            bindexpr: Box::new(XPD::Expr::from(*self.bindexpr)),
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
