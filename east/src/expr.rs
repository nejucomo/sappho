use sappho_effect::Effect;
use sappho_kast::ObjectDef;

use crate::EastProvider;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr<FX>
where
    FX: Effect,
{
    ObjectDef(ObjectDef<EastProvider, FX>),
}
