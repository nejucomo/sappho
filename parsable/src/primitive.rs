//! A set of primitive [Parser] impls
//!
//! These are below the level of [Parsable](crate::Parsable) and represent value-less syntax (such as whitespace).
pub use chumsky::primitive::just;
use chumsky::Parser as _;

use crate::Parser;

/// Parse sappho-specific whitespace
///
/// In sappho, only ' ' and '\n' are allowed as syntactic whitespace, and '\t', '\r', or non-ASCII whitespace is rejected.
pub fn space() -> impl Parser<()> {
    // TODO: This seems potentially more efficient but regresses error messages on expectations:
    // chumsky::primitive::filter(|&c| c == ' ' || c == '\n').ignored().repeated().ignored()

    just(' ').or(just('\n')).repeated().ignored()
}

pub fn bracketed<P, O>([open, close]: [char; 2], inner: P) -> impl Parser<O>
where
    P: Parser<O>,
{
    just(open)
        .then_opt_space()
        .ignore_then(inner)
        .then_ignore(space().or_not().ignore_then(just(close)))
}
