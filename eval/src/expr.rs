mod application;
mod core;
mod effects;
mod letexpr;
mod literal;
mod lookup;
mod matchexpr;
mod object;

use crate::{Eval, Result};
use sappho_east::{EffectExpr, Expr};
use sappho_unparse::Unparse;
use sappho_value::{ScopeRef, ValRef};

impl<FX> Eval for Expr<FX>
where
    EffectExpr<FX>: Eval,
    FX: Unparse,
{
    fn eval(&self, scope: &ScopeRef) -> Result<ValRef> {
        log::debug!("Evaluating:\n  From: {}\n  ...\n", self);
        let r = eval_expr(self, scope);
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

fn eval_expr<FX>(expr: &Expr<FX>, scope: &ScopeRef) -> Result<ValRef>
where
    EffectExpr<FX>: Eval,
    FX: Unparse,
{
    use Expr::*;

    match expr {
        Core(x) => x.eval(scope),
        Match(x) => x.eval(scope),
        Application(x) => x.eval(scope),
        Lookup(x) => x.eval(scope),
    }
}
