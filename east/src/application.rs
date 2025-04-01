use sappho_effect::Effect;

use crate::BoxWise;

#[derive(Clone, Debug, PartialEq)]
pub struct Application<FX>
where
    FX: Effect,
{
    target: BoxWise<FX>,
    argument: BoxWise<FX>,
}
