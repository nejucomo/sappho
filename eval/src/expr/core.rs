use crate::{Eval, Result};
use sappho_ast_effect::Effect;
use sappho_ast_kernel::{EffectExpr, Kernel};
use sappho_ast_red::AstRed;
use sappho_value::{ScopeRef, ValRef};

impl<FX> Eval for Kernel<AstRed, FX>
where
    EffectExpr<AstRed, FX>: Eval,
    FX: Effect,
{
    fn eval(&self, scope: &ScopeRef) -> Result<ValRef> {
        use sappho_ast_kernel::Kernel::*;

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
