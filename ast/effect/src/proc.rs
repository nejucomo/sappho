use sappho_unparse::{Stream, Unparse};

use crate::{Effect, EffectDescription, QueryEffect};

use self::ProcEffect::*;

/// A proc effect can either be a mutation or a query effect.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ProcEffect {
    /// Inquire is identical to [QueryEffect::Inquire](crate::QueryEffect::Inquire).
    Inquire,

    /// Evoke a mutation, as in `!exit`.
    Invoke,
}

impl Effect for ProcEffect {
    fn context() -> &'static str {
        "proc"
    }

    fn description(self) -> EffectDescription {
        match self {
            Inquire => QueryEffect::Inquire.description(),
            Invoke => EffectDescription {
                sigil: "!",
                noun: "invocation",
                verb: "invoke",
            },
        }
    }
}

impl Unparse for ProcEffect {
    fn unparse_into(&self, s: &mut Stream) {
        s.write(self.description().sigil);
    }
}
