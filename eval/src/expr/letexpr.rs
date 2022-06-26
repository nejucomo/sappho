use crate::{Eval, Result};
use sappho_east::LetExpr;
use sappho_value::{ScopeRef, ValRef};

impl<FX> Eval for LetExpr<FX>
where
    FX: Eval,
{
    fn eval(&self, scope: &ScopeRef) -> Result<ValRef> {
        let subscope =
            self.clauses
                .iter()
                .try_fold(scope.clone(), |sc, clause| -> Result<ScopeRef> {
                    let bindval = clause.bindexpr.eval(&sc)?;
                    let subscope = sc.bind(&clause.binding, &bindval)?;
                    Ok(subscope)
                })?;

        self.tail.eval(&subscope)
    }
}
