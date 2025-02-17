mod unpack;

use crate::lfreduce::reduce_listform;
use crate::{Identifier, Literal};
use sappho_ast as ast;
use sappho_attrs::{Attrs, TryIntoAttrs};
use sappho_unparse::{Stream, Unparse};
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
            ast::Pattern::List(x) => x.into(),
        }
    }
}

impl From<ast::ListPattern> for Pattern {
    fn from(alp: ast::ListPattern) -> Pattern {
        use Pattern::Unpack;

        reduce_listform(
            alp,
            Unpack(UnpackPattern::default()),
            Pattern::Bind,
            Pattern::from,
            |unpack| Unpack(unpack.into_iter().collect()),
        )
    }
}

impl From<Pattern> for ast::Pattern {
    fn from(p: Pattern) -> Self {
        use Pattern::*;

        match p {
            Bind(x) => ast::Pattern::Bind(x),
            LitEq(x) => ast::Pattern::LitEq(x),
            Unpack(x) => x
                .as_list_form()
                .and_then(|listform| {
                    listform
                        .into_iter()
                        .map(|ei| {
                            ei.cloned()
                                .map_left(|p| Ok(ast::Pattern::from(p)))
                                .map_right(|t| match t {
                                    Bind(b) => Ok(b),
                                    _ => {
                                        // Non-empty, non-Bind tails disallowed:
                                        Err(())
                                    }
                                })
                                .factor_err()
                        })
                        .collect::<Result<_, _>>()
                        .ok()
                        .map(ast::Pattern::List)
                })
                .unwrap_or_else(|| ast::Pattern::Unpack(x.into())),
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
        }
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unparse().fmt(f)
    }
}

#[cfg(test)]
mod tests;
