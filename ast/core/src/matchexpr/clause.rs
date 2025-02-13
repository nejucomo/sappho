use sappho_ast_effect::Effect;
use sappho_unparse::{Stream, Unparse};

use crate::{AstProvider, AstTransformInto, BoxExpr};

/// A `match` clause, ie `3 -> 0` and `y -> y` in `match x { 3 -> 0, y -> y }`.
#[derive(Debug, derive_new::new)]
pub struct MatchClause<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    /// The binding pattern, ie `3` in `3 -> 0` and the first `y` in `y -> y`.
    pub pattern: XP::Pattern,

    /// The match body expression, ie `0` in `3 -> 0` and the second `y` in `y -> y`.
    pub body: BoxExpr<XP, FX>,
}

impl<XPS, XPD, FX> AstTransformInto<MatchClause<XPD, FX>> for MatchClause<XPS, FX>
where
    XPD: AstProvider,
    XPS: AstProvider,
    XPS::Pattern: AstTransformInto<XPD::Pattern>,
    XPS::Expr<FX>: AstTransformInto<XPD::Expr<FX>>,
    FX: Effect,
{
    fn ast_transform(self) -> MatchClause<XPD, FX> {
        MatchClause {
            pattern: self.pattern.ast_transform(),
            body: self.body.ast_transform(),
        }
    }
}

impl<XP, FX> Unparse for MatchClause<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn unparse_into(&self, s: &mut Stream) {
        s.write(&self.pattern);
        s.write(" -> ");
        s.write(&self.body);
    }
}

impl<XP, FX> Clone for MatchClause<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn clone(&self) -> Self {
        MatchClause::new(self.pattern.clone(), self.body.clone())
    }
}

impl<XP, FX> PartialEq for MatchClause<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn eq(&self, other: &Self) -> bool {
        self.pattern == other.pattern && self.body == other.body
    }
}
