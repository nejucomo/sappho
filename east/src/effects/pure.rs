use crate::Expr;
use crate::FromFx;

pub use sappho_ast::PureEffects;
pub type PureExpr = Expr<PureEffects>;

impl FromFx for PureEffects {
    type AstFx = Self;

    fn from_fx(astfx: Self) -> Self {
        astfx
    }
}
