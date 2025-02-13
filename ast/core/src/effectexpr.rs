use sappho_ast_effect::Effect;
use sappho_unparse::{Stream, Unparse};

use crate::{AstProvider, AstTransformInto, BoxExpr};

#[derive(Debug, derive_new::new)]
pub struct EffectExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    pub effect: FX,
    pub expr: BoxExpr<XP, FX>,
}

impl<XPS, XPD, FX> AstTransformInto<EffectExpr<XPD, FX>> for EffectExpr<XPS, FX>
where
    XPD: AstProvider,
    XPS: AstProvider,
    XPS::Expr<FX>: AstTransformInto<XPD::Expr<FX>>,
    FX: Effect,
{
    fn ast_transform(self) -> EffectExpr<XPD, FX> {
        EffectExpr {
            effect: self.effect,
            expr: self.expr.ast_transform(),
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
