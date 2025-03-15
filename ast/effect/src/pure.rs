use chumsky::prelude::empty;
use indoc::indoc;
use sappho_parsable::{Parsable, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::Effect;

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

/// This is subtle: we need to construct the parser for general code, but it always produces a value, since [PureEffect] is a phantom type.
impl Parsable for PureEffect {
    fn parser() -> impl Parser<Self> {
        empty().err_ez(indoc! {
            r#"
            Internal Parser bug: an attempt was made to parse the "pure
            effect" which is a phantom type; there is no such thing in
            the sappho syntax
            "#
        })
    }
}

impl Unparse for PureEffect {
    fn unparse_into(&self, _s: &mut Stream) {
        unreachable!("pure effects are never instantiated");
    }
}
