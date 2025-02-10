use sappho_ast_effect::Effect;
use sappho_unparse::{Stream, Unparse};

use crate::ExprProvider;

#[derive(Clone, Debug, PartialEq, derive_new::new)]
pub struct EffectExpr<XP, FX>
where
    XP: ExprProvider,
    FX: Effect,
{
    pub effect: FX,
    pub expr: Box<XP::Expr<FX>>,
}

impl<XP, FX> EffectExpr<XP, FX>
where
    XP: ExprProvider,
    FX: Effect,
{
    pub fn transform_into<XPD>(self) -> EffectExpr<XPD, FX>
    where
        XPD: ExprProvider,
        XPD::Expr<FX>: From<XP::Expr<FX>>,
    {
        EffectExpr {
            effect: self.effect,
            expr: Box::new(XPD::Expr::from(*self.expr)),
        }
    }
}

impl<XP, FX> Unparse for EffectExpr<XP, FX>
where
    XP: ExprProvider,
    FX: Effect,
{
    fn unparse_into(&self, s: &mut Stream) {
        s.write(&self.effect);
        s.write(&self.expr);
    }
}
