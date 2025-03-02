use sappho_ast_core::Literal;
use sappho_attrs::Attrs;
use sappho_identifier::RcId;
use sappho_unparse::{Stream, Unparse};
use std::fmt;

#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub enum Pattern {
    Bind(RcId),
    LitEq(Literal),
    Unpack(Attrs<Pattern>),
}

impl Unparse for Pattern {
    fn unparse_into(&self, s: &mut Stream) {
        use Pattern::*;

        match self {
            Bind(x) => x.unparse_into(s),
            LitEq(x) => x.unparse_into(s),
            Unpack(x) => x.unparse_into(s),
        }
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unparse().fmt(f)
    }
}
