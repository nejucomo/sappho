use sappho_east::Wise;
use sappho_effect::Effect;
use sappho_value::{Scope, Value};

use self::Step::*;

#[derive(Debug)]
pub(crate) enum Step<'a, FX, C>
where
    FX: Effect,
{
    Produce(Value),
    Continue(Scope, &'a Wise<FX>, Option<C>),
}

impl<'a, FX, C> Step<'a, FX, C>
where
    FX: Effect,
{
    pub(crate) fn map_cont<F, C2>(self, f: F) -> Step<'a, FX, C2>
    where
        F: FnOnce(C) -> C2,
    {
        match self {
            Produce(v) => Produce(v),
            Continue(s, x, optc) => Continue(s, x, optc.map(f)),
        }
    }

    pub(crate) fn cont_from<C2>(self) -> Step<'a, FX, C2>
    where
        C2: From<C>,
    {
        self.map_cont(C2::from)
    }
}
