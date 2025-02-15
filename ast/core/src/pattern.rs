use crate::{AstTransformInto, Identifier, Literal};
use sappho_identmap::{HeadTailUnrollable, IdentMap, TryIntoIdentMap};
use sappho_listform::ListForm;
use sappho_unparse::{Stream, Unparse};
use std::fmt;

#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub enum CorePattern {
    Bind(Identifier),
    LitEq(Literal),
    Unpack(IdentMap<CorePattern>),
}

impl<'a> From<&'a str> for CorePattern {
    fn from(s: &'a str) -> Self {
        CorePattern::Bind(s.to_string())
    }
}

impl AstTransformInto<CorePattern> for CorePattern {
    fn ast_transform(self) -> CorePattern {
        self
    }
}

impl<X, T> AstTransformInto<CorePattern> for ListForm<X, T>
where
    CorePattern: From<T> + From<X>,
{
    fn ast_transform(self) -> CorePattern {
        self.unroll_via_froms()
    }
}

impl Unparse for CorePattern {
    fn unparse_into(&self, s: &mut Stream) {
        use CorePattern::*;

        match self {
            Bind(x) => x.unparse_into(s),
            LitEq(x) => x.unparse_into(s),
            Unpack(x) => x.unparse_into(s),
        }
    }
}

impl fmt::Display for CorePattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unparse().fmt(f)
    }
}

impl TryIntoIdentMap<CorePattern> for CorePattern {
    fn try_into_identmap(&self) -> Option<&IdentMap<CorePattern>> {
        match self {
            CorePattern::Unpack(up) => Some(up),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests;
