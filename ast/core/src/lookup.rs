use crate::{ExprProvider, Identifier};
use sappho_ast_effect::Effect;
use sappho_unparse::{Stream, Unparse};

/// An attribute lookup expression, ie: `x.foo`.
#[derive(Debug, derive_new::new)]
pub struct LookupExpr<XP, FX>
where
    XP: ExprProvider,
    FX: Effect,
{
    /// The target expression of the lookup, ie `x` in `x.foo`.
    pub target: Box<XP::Expr<FX>>,

    /// An attribute name, ie: `foo` in `x.foo`.
    pub attr: Identifier,
}

impl<XP, FX> LookupExpr<XP, FX>
where
    XP: ExprProvider,
    FX: Effect,
{
    pub fn transform_into<XPD>(self) -> LookupExpr<XPD, FX>
    where
        XPD: ExprProvider,
        XPD::Expr<FX>: From<XP::Expr<FX>>,
    {
        LookupExpr {
            target: Box::new(XPD::Expr::from(*self.target)),
            attr: self.attr,
        }
    }
}

impl<XP, FX> Unparse for LookupExpr<XP, FX>
where
    XP: ExprProvider,
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
    XP: ExprProvider,
    FX: Effect,
{
    fn clone(&self) -> Self {
        LookupExpr::new(self.target.clone(), self.attr.clone())
    }
}

impl<XP, FX> PartialEq for LookupExpr<XP, FX>
where
    XP: ExprProvider,
    FX: Effect,
{
    fn eq(&self, other: &Self) -> bool {
        self.attr == other.attr && self.target == other.target
    }
}
