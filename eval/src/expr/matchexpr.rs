use crate::{Eval, Result};
use sappho_ast_core::{EffectExpr, MatchExpr};
use sappho_ast_effect::Effect;
use sappho_ast_reduced::AstRed;
use sappho_value::{ScopeRef, ValRef};

impl<FX> Eval for MatchExpr<AstRed, FX>
where
    EffectExpr<AstRed, FX>: Eval,
    FX: Effect,
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
