use sappho_effect::Effect;

use crate::BoxWise;

#[derive(Clone, Debug, PartialEq)]
pub struct Interaction<FX>
where
    FX: Effect,
{
    effect: FX,
    target: BoxWise<FX>,
}
