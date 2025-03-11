use sappho_unparse::{Stream, Unparse};

use crate::{Effect, ProcEffect, QueryEffect};

/// Pure effects cannot be instantiated, because pure expressions have no side effects.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PureEffect {}

impl Effect for PureEffect {
    fn context() -> &'static str {
        "pure"
    }

    fn description(self) -> crate::EffectDescription {
        unreachable!("pure effects are never instantiated");
    }
}

impl TryFrom<QueryEffect> for PureEffect {
    type Error = QueryEffect;

    fn try_from(value: QueryEffect) -> Result<Self, Self::Error> {
        Err(value)
    }
}

impl TryFrom<ProcEffect> for PureEffect {
    type Error = ProcEffect;

    fn try_from(value: ProcEffect) -> Result<Self, Self::Error> {
        Err(value)
    }
}

impl Unparse for PureEffect {
    fn unparse_into(&self, _s: &mut Stream) {
        unreachable!("pure effects are never instantiated");
    }
}
