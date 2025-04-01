use sappho_effect::Effect;
use sappho_identifier::RcId;

use crate::BoxWise;

#[derive(Clone, Debug, PartialEq)]
pub struct Lookup<FX>
where
    FX: Effect,
{
    target: BoxWise<FX>,
    attrname: RcId,
}
