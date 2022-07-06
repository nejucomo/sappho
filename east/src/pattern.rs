mod unpack;

use crate::{Identifier, Literal};
use sappho_ast as ast;
use sappho_identmap::{IdentMap, TryIntoIdentMap};
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

        let tailpat = alp
            .tail
            .map(Pattern::Bind)
            .unwrap_or_else(|| Unpack(UnpackPattern::default()));

        alp.body.into_iter().rev().fold(tailpat, |tail, head| {
            Unpack(UnpackPattern::from_iter([
                ("head".to_string(), Pattern::from(head)),
                ("tail".to_string(), tail),
            ]))
        })
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
                .and_then(|(pats, tailpat)| {
                    let tailbind = match tailpat {
                        Some(Bind(b)) => Some(b.to_string()),
                        None => None,
                        Some(_) => {
                            // Non-empty, non-Bind tails disallowed:
                            return None;
                        }
                    };
                    Some(
                        ast::ListPattern::new(
                            pats.into_iter().map(|p| ast::Pattern::from(p.clone())),
                            tailbind,
                        )
                        .into(),
                    )
                })
                .unwrap_or_else(|| ast::Pattern::Unpack(x.into())),
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

impl TryIntoIdentMap<Pattern> for Pattern {
    fn try_into_identmap(&self) -> Option<&IdentMap<Pattern>> {
        use std::ops::Deref;

        match self {
            Pattern::Unpack(up) => Some(up.deref()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests;
