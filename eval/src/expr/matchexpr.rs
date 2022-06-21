use crate::{bind, Eval, Result};
use sappho_east::MatchExpr;
use sappho_value::{ScopeRef, ValRef};

impl<FX> Eval for MatchExpr<FX>
where
    FX: Eval,
{
    fn eval(&self, scope: &ScopeRef) -> Result<ValRef> {
        use crate::Error::Mismatch;

        let MatchExpr { target, clauses } = &self;

        let tval = target.eval(scope)?;
        for clause in clauses {
            if let Some(matchscope) = bind(&clause.pattern, &tval, scope) {
                return clause.body.eval(&matchscope);
            }
        }

        Err(Mismatch(
            tval,
            clauses.iter().map(|c| c.pattern.clone()).collect(),
        ))
    }
}
