mod unpack;

use crate::{Identifier, Literal};
use sappho_ast as ast;
use std::fmt;

pub use self::unpack::UnpackPattern;

#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub enum Pattern {
    Bind(Identifier),
    LitEq(Literal),
    Unpack(UnpackPattern),
}

impl From<ast::Pattern> for Pattern {
    fn from(ap: ast::Pattern) -> Self {
        use Pattern::*;

        match ap {
            ast::Pattern::Bind(x) => Bind(x),
            ast::Pattern::LitEq(x) => LitEq(x),
            ast::Pattern::Unpack(x) => Unpack(x.into()),
            ast::Pattern::List(x) => Unpack(UnpackPattern::from(x)),
        }
    }
}

impl From<Pattern> for ast::Pattern {
    fn from(p: Pattern) -> Self {
        use Pattern::*;

        match p {
            Bind(x) => ast::Pattern::Bind(x),
            LitEq(x) => ast::Pattern::LitEq(x),
            Unpack(x) => ast::Pattern::Unpack(x.into()),
        }
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Pattern::*;

        match self {
            Bind(x) => x.fmt(f),
            LitEq(x) => x.fmt(f),
            Unpack(x) => x.fmt(f),
        }
    }
}

#[cfg(test)]
mod tests;
