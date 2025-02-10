use sappho_ast_effect::Effect;
use sappho_unparse::{Stream, Unparse};

use crate::ExprProvider;

/// A `match` clause, ie `3 -> 0` and `y -> y` in `match x { 3 -> 0, y -> y }`.
#[derive(Debug, derive_new::new)]
pub struct MatchClause<XP, FX>
where
    XP: ExprProvider,
    FX: Effect,
{
    /// The binding pattern, ie `3` in `3 -> 0` and the first `y` in `y -> y`.
    pub pattern: XP::Pattern,

    /// The match body expression, ie `0` in `3 -> 0` and the second `y` in `y -> y`.
    pub body: Box<XP::Expr<FX>>,
}

impl<XP, FX> MatchClause<XP, FX>
where
    XP: ExprProvider,
    FX: Effect,
{
    pub fn transform_into<XPD>(self) -> MatchClause<XPD, FX>
    where
        XPD: ExprProvider,
        XPD::Pattern: From<XP::Pattern>,
        XPD::Expr<FX>: From<XP::Expr<FX>>,
    {
        MatchClause {
            pattern: XPD::Pattern::from(self.pattern),
            body: Box::new(XPD::Expr::from(*self.body)),
        }
    }
}

impl<XP, FX> Unparse for MatchClause<XP, FX>
where
    XP: ExprProvider,
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
    XP: ExprProvider,
    FX: Effect,
{
    fn clone(&self) -> Self {
        MatchClause::new(self.pattern.clone(), self.body.clone())
    }
}

impl<XP, FX> PartialEq for MatchClause<XP, FX>
where
    XP: ExprProvider,
    FX: Effect,
{
    fn eq(&self, other: &Self) -> bool {
        self.pattern == other.pattern && self.body == other.body
    }
}
