use derive_new::new;
use sappho_effect::Effect;

use crate::Expr;

#[derive(Clone, Debug, PartialEq, new)]
pub struct Application<FX>
where
    FX: Effect,
{
    #[new(into)]
    pub target: Box<Expr<FX>>,
    #[new(into)]
    pub argument: Box<Expr<FX>>,
}
