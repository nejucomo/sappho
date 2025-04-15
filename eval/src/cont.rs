use derive_more::From;
use sappho_effect::Effect;

#[derive(Debug, From)]
pub enum Cont<'x, FX>
where
    FX: Effect,
{
    Fixme(&'x FX),
}
