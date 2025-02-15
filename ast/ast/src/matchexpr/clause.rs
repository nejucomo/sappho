use sappho_ast_effect::Effect;
use sappho_unparse::{Stream, Unparse};

use crate::{Expr, Pattern};

/// A `match` clause, ie `3 -> 0` and `y -> y` in `match x { 3 -> 0, y -> y }`.
#[derive(Clone, Debug, PartialEq, derive_new::new)]
pub struct MatchClause<FX>
where
    FX: Effect,
{
    /// The binding pattern, ie `3` in `3 -> 0` and the first `y` in `y -> y`.
    pub pattern: Pattern,

    /// The match body expression, ie `0` in `3 -> 0` and the second `y` in `y -> y`.
    pub body: Box<Expr<FX>>,
}

impl<FX> Unparse for MatchClause<FX>
where
    FX: Effect,
{
    fn unparse_into(&self, s: &mut Stream) {
        s.write(&self.pattern);
        s.write(" -> ");
        s.write(&self.body);
    }
}
