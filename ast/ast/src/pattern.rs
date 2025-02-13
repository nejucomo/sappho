use sappho_ast_core::CorePattern;
use sappho_identmap::IdentMap;
use sappho_listform::ListForm;
use sappho_unparse::{Stream, Unparse};

use crate::{Identifier, Literal};

pub type ListPattern = ListForm<Pattern, Identifier>;

#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub enum Pattern {
    Bind(Identifier),
    LitEq(Literal),
    Unpack(IdentMap<Pattern>),
    List(ListPattern),
}

impl From<Pattern> for CorePattern {
    fn from(p: Pattern) -> Self {
        match p {
            Pattern::Bind(x) => CorePattern::Bind(x),
            Pattern::LitEq(x) => CorePattern::LitEq(x),
            Pattern::Unpack(x) => CorePattern::Unpack(x.into_map_values(Pattern::into)),
            Pattern::List(x) => x.into(),
        }
    }
}

impl Unparse for Pattern {
    fn unparse_into(&self, s: &mut Stream) {
        use Pattern::*;

        match self {
            Bind(x) => x.unparse_into(s),
            LitEq(x) => x.unparse_into(s),
            Unpack(x) => x.unparse_into(s),
            List(x) => x.unparse_into(s),
        }
    }
}
