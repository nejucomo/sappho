use crate::{Eval, Result};
use sappho_east::{CoreExpr, EffectExpr};
use sappho_unparse::Unparse;
use sappho_value::{ScopeRef, ValRef};

impl<FX> Eval for CoreExpr<FX>
where
    EffectExpr<FX>: Eval,
    FX: Unparse,
{
    fn eval(&self, scope: &ScopeRef) -> Result<ValRef> {
        use sappho_astcore::CoreExpr::*;

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
