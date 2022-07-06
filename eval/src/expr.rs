mod application;
mod effects;
mod letexpr;
mod literal;
mod lookup;
mod matchexpr;
mod object;

use crate::{Eval, Result};
use sappho_east::GenExpr;
use sappho_value::{ScopeRef, ValRef};

impl<FX> Eval for GenExpr<FX>
where
    FX: Eval + std::fmt::Display,
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

fn eval_expr<FX>(expr: &GenExpr<FX>, scope: &ScopeRef) -> Result<ValRef>
where
    FX: Eval + std::fmt::Display,
{
    use GenExpr::*;

    match expr {
        Lit(x) => x.eval(scope),
        Ref(x) => {
            let v = scope.deref(x)?;
            Ok(v)
        }
        Object(x) => x.eval(scope),
        Let(x) => x.eval(scope),
        Match(x) => x.eval(scope),
        Application(x) => x.eval(scope),
        Lookup(x) => x.eval(scope),
        Effect(x) => x.eval(scope),
    }
}
