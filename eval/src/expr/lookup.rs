use crate::{Eval, Result};
use sappho_east::LookupExpr;
use sappho_unparse::DisplayDepth;
use sappho_value::{ScopeRef, ValRef};

impl<FX> Eval for LookupExpr<FX>
where
    FX: Eval + DisplayDepth,
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
