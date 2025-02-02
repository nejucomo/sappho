use crate::{Eval, Result};
use sappho_ast_reduced::{CoreExpr, EffectExpr};
use sappho_value::{ScopeRef, ValRef};

impl<FX> Eval for CoreExpr<FX>
where
    EffectExpr<FX>: Eval,
{
    fn eval(&self, scope: &ScopeRef) -> Result<ValRef> {
        use sappho_ast_core::CoreExpr::*;

        match self {
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
}
