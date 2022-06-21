use crate::{Eval, Result};
use sappho_east::LetExpr;
use sappho_value::{ScopeRef, ValRef};

impl<FX> Eval for LetExpr<FX>
where
    FX: Eval,
{
    fn eval(&self, scope: &ScopeRef) -> Result<ValRef> {
        let LetExpr {
            binding,
            bindexpr,
            tail,
        } = &self;

        let bindval = bindexpr.eval(scope)?;
        let subscope = scope.extend(binding, bindval);

        tail.eval(&subscope)
    }
}
