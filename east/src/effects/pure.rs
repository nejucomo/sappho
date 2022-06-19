use crate::FromFx;
use crate::GenExpr;

pub use sappho_ast::PureEffects;
pub type PureExpr = GenExpr<PureEffects>;

impl FromFx for PureEffects {
    type AstFx = Self;

    fn from_fx(astfx: Self) -> Self {
        astfx
    }
}
