use sappho_ast_effect::Effect;
use sappho_unparse::{Stream, Unparse};

use crate::{AstProvider, AstTransformInto, BoxExpr, Identifier};

/// An attribute lookup expression, ie: `x.foo`.
#[derive(Debug, derive_new::new)]
pub struct LookupExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    /// The target expression of the lookup, ie `x` in `x.foo`.
    pub target: BoxExpr<XP, FX>,

    /// An attribute name, ie: `foo` in `x.foo`.
    pub attr: Identifier,
}

impl<XPS, XPD, FX> AstTransformInto<LookupExpr<XPD, FX>> for LookupExpr<XPS, FX>
where
    XPD: AstProvider,
    XPS: AstProvider,
    XPS::Expr<FX>: AstTransformInto<XPD::Expr<FX>>,
    FX: Effect,
{
    fn ast_transform(self) -> LookupExpr<XPD, FX> {
        LookupExpr {
            target: self.target.ast_transform(),
            attr: self.attr,
        }
    }
}

impl<XP, FX> Unparse for LookupExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn unparse_into(&self, s: &mut Stream) {
        s.write(&self.target);
        s.write(".");
        s.write(&self.attr);
    }
}

impl<XP, FX> Clone for LookupExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn clone(&self) -> Self {
        LookupExpr::new(self.target.clone(), self.attr.clone())
    }
}

impl<XP, FX> PartialEq for LookupExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn eq(&self, other: &Self) -> bool {
        self.attr == other.attr && self.target == other.target
    }
}
