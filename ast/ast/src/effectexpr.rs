use sappho_ast_effect::Effect;
use sappho_unparse::{Stream, Unparse};

use crate::Expr;

#[derive(Clone, Debug, PartialEq, derive_new::new)]
pub struct EffectExpr<FX>
where
    FX: Effect,
{
    pub effect: FX,
    pub expr: Box<Expr<FX>>,
}

impl<FX> Unparse for EffectExpr<FX>
where
    FX: Effect,
{
    fn unparse_into(&self, s: &mut Stream) {
        s.write(&self.effect);
        s.write(&self.expr);
    }
}
