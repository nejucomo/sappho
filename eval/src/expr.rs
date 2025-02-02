mod application;
mod core;
mod effects;
mod letexpr;
mod literal;
mod lookup;
mod matchexpr;
mod object;

use crate::{Eval, Result};
use sappho_ast_reduced::{EffectExpr, Expr};
use sappho_value::{ScopeRef, ValRef};

impl<FX> Eval for Expr<FX>
where
    EffectExpr<FX>: Eval,
{
    fn eval(&self, scope: &ScopeRef) -> Result<ValRef> {
        use std::ops::Deref;

        log::debug!("Evaluating:\n  From: {}\n  ...\n", self);
        let r = self.deref().eval(scope);
        log::debug!(
            "Evaluated:\n  From: {}\n  To: {}\n",
            self,
            match &r {
                Ok(v) => v.to_string(),
                Err(e) => format!("{:?}", e),
            }
        );
        r
    }
}
