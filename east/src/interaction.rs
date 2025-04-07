use derive_new::new;
use sappho_effect::Effect;

use crate::BoxWise;

#[derive(Clone, Debug, PartialEq, new)]
pub struct Interaction<FX>
where
    FX: Effect,
{
    #[new(into)]
    pub effect: FX,
    #[new(into)]
    pub target: BoxWise<FX>,
}
