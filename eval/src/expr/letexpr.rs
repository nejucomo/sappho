use crate::{Eval, Result};
use sappho_ast_reduced::{EffectExpr, LetExpr};
use sappho_value::{ScopeRef, ValRef};

impl<FX> Eval for LetExpr<FX>
where
    EffectExpr<FX>: Eval,
{
    fn eval(&self, scope: &ScopeRef) -> Result<ValRef> {
        // Declare all forward bindings first:
        let subscope = scope.declare(self.clauses.iter().map(|clause| &clause.binding));

        // Now fulfill the definitions for each clause:
        for clause in self.clauses.iter() {
            let v = clause.bindexpr.eval(&subscope)?;
            subscope.bind_pattern(&clause.binding, &v)?;
        }

        self.tail.eval(&subscope)
    }
}
