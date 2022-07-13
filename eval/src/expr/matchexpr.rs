use crate::{Eval, Result};
use sappho_east::MatchExpr;
use sappho_unparse::{Unparse, Stream};
use sappho_value::{ScopeRef, ValRef};

impl<FX> Eval for MatchExpr<FX>
where
    FX: Eval + Unparse,
{
    fn eval(&self, scope: &ScopeRef) -> Result<ValRef> {
        use crate::Error::Mismatch;

        let MatchExpr { target, clauses } = &self;

        let tval = target.eval(scope)?;
        for clause in clauses {
            if let Ok(matchscope) = scope.declare_then_bind(&clause.pattern, &tval) {
                return clause.body.eval(&matchscope);
            }
            // TODO: Verify any Err case is a benign mismatch.
        }

        Err(Mismatch(
            tval,
            clauses.iter().map(|c| c.pattern.clone()).collect(),
        ))
    }
}
