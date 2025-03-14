use sappho_ast_effect::Effect;
use sappho_unparse::{Stream, Unparse};

use crate::AstProvider;

#[derive(Debug, derive_new::new)]
pub struct Interaction<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    pub effect: FX,
    pub expr: Box<XP::Expr<FX>>,
}

impl<XP, FX> Interaction<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    pub fn transform_into<XPD>(self) -> Interaction<XPD, FX>
    where
        XPD: AstProvider,
        XPD::Expr<FX>: From<XP::Expr<FX>>,
    {
        Interaction {
            effect: self.effect,
            expr: Box::new(XPD::Expr::from(*self.expr)),
        }
    }
}

impl<XP, FX> Unparse for Interaction<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn unparse_into(&self, s: &mut Stream) {
        s.write(&self.effect);
        s.write(&self.expr);
    }
}

impl<XP, FX> Clone for Interaction<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn clone(&self) -> Self {
        Interaction::new(self.effect, self.expr.clone())
    }
}

impl<XP, FX> PartialEq for Interaction<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn eq(&self, other: &Self) -> bool {
        self.effect == other.effect && self.expr == other.expr
    }
}
