use chumsky::prelude::just;
use chumsky::Parser as _;
use sappho_parsable::{Parsable, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::{Effect, EffectDescription};

use self::QueryEffect::Inquire;

/// The query effect reads mutable memory.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum QueryEffect {
    /// Inquire is the name of the `$myvar` effect syntax & semantics.
    Inquire,
}

impl Effect for QueryEffect {
    fn context() -> &'static str {
        "query"
    }

    fn description(self) -> EffectDescription {
        EffectDescription {
            sigil: "$",
            noun: "inquiry",
            verb: "inquire-of",
        }
    }
}

impl Parsable for QueryEffect {
    fn parser() -> impl Parser<Self> {
        just(Inquire.sigil()).to(Inquire)
    }
}

impl Unparse for QueryEffect {
    fn unparse_into(&self, s: &mut Stream) {
        s.write(self.sigil());
    }
}
