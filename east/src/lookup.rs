use derive_new::new;
use sappho_effect::Effect;
use sappho_identifier::RcId;

use crate::BoxWise;

#[derive(Clone, Debug, PartialEq, new)]
pub struct Lookup<FX>
where
    FX: Effect,
{
    #[new(into)]
    pub target: BoxWise<FX>,
    #[new(into)]
    pub attrname: RcId,
}
