use sappho_ast_effect::Effect;
use sappho_unparse::{Stream, Unparse};

use crate::{AstProvider, CmtExpr};

#[derive(Debug, derive_new::new)]
pub struct EffectExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    pub effect: FX,
    pub expr: CmtExpr<XP, FX>,
}

impl<XP, FX> EffectExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    pub fn transform_into<XPD>(self) -> EffectExpr<XPD, FX>
    where
        XPD: AstProvider,
        CmtExpr<XPD, FX>: From<CmtExpr<XP, FX>>,
    {
        EffectExpr {
            effect: self.effect,
            expr: CmtExpr::from(self.expr),
        }
    }
}

impl<XP, FX> Unparse for EffectExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn unparse_into(&self, s: &mut Stream) {
        s.write(&self.effect);
        s.write(&self.expr);
    }
}

impl<XP, FX> Clone for EffectExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn clone(&self) -> Self {
        EffectExpr::new(self.effect, self.expr.clone())
    }
}

impl<XP, FX> PartialEq for EffectExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn eq(&self, other: &Self) -> bool {
        self.effect == other.effect && self.expr == other.expr
    }
}
