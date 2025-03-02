use crate::{Eval, Result};
use sappho_ast::Effect;
use sappho_ast_core::{EffectExpr, LookupExpr};
use sappho_ast_reduced::AstRed;
use sappho_value::{ScopeRef, ValRef};

impl<FX> Eval for LookupExpr<AstRed, FX>
where
    EffectExpr<AstRed, FX>: Eval,
    FX: Effect,
{
    fn eval(&self, scope: &ScopeRef) -> Result<ValRef> {
        use crate::Error::MissingAttr;
        use sappho_value::AttrsVals;

        let LookupExpr { target, attr } = self;
        let tval = target.eval(scope)?;
        let attrs: &AttrsVals = tval.coerce()?;
        attrs
            .get(attr)
            .cloned()
            .ok_or_else(|| MissingAttr(tval, attr.clone()))
    }
}
