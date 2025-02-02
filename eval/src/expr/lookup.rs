use crate::{Eval, Result};
use sappho_ast_reduced::{EffectExpr, LookupExpr};
use sappho_value::{ScopeRef, ValRef};

impl<FX> Eval for LookupExpr<FX>
where
    EffectExpr<FX>: Eval,
{
    fn eval(&self, scope: &ScopeRef) -> Result<ValRef> {
        use crate::Error::MissingAttr;
        use sappho_value::Attrs;

        let LookupExpr { target, attr } = self;
        let tval = target.eval(scope)?;
        let attrs: &Attrs = tval.coerce()?;
        attrs
            .get(attr)
            .cloned()
            .ok_or_else(|| MissingAttr(tval, attr.clone()))
    }
}
