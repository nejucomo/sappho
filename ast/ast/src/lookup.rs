use sappho_ast_effect::Effect;
use sappho_unparse::{Stream, Unparse};

use crate::{Expr, Identifier};

/// An attribute lookup expression, ie: `x.foo`.
#[derive(Clone, Debug, PartialEq, derive_new::new)]
pub struct LookupExpr<FX>
where
    FX: Effect,
{
    /// The target expression of the lookup, ie `x` in `x.foo`.
    pub target: Box<Expr<FX>>,

    /// An attribute name, ie: `foo` in `x.foo`.
    pub attr: Identifier,
}

impl<FX> Unparse for LookupExpr<FX>
where
    FX: Effect,
{
    fn unparse_into(&self, s: &mut Stream) {
        s.write(&self.target);
        s.write(".");
        s.write(&self.attr);
    }
}
