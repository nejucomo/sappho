use sappho_effect::Effect;
use sappho_kast::KastProvider;

use crate::Expr;

#[derive(Clone, Debug, PartialEq)]
pub struct BeastProvider;

impl KastProvider for BeastProvider {
    type Expr<FX>
        = Expr<FX>
    where
        FX: Effect;
}
