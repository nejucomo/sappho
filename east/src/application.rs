use derive_new::new;
use sappho_effect::Effect;

use crate::BoxWise;

#[derive(Clone, Debug, PartialEq, new)]
pub struct Application<FX>
where
    FX: Effect,
{
    #[new(into)]
    pub target: BoxWise<FX>,
    #[new(into)]
    pub argument: BoxWise<FX>,
}
