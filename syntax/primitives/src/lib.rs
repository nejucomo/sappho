//! A set of primitive [Parser] impls
//!
//! These are below the level of [Parsable](sappho_syntax_parsable::Parsable) and represent value-less syntax (such as whitespace).
use chumsky::prelude::filter;
use chumsky::Parser as _;
use sappho_syntax_parsable::Parser;

/// Parse sappho-specific whitespace
///
/// In sappho, only ' ' and '\n' are allowed as syntactic whitespace, and '\t', '\r', or non-ASCII whitespace is rejected.
pub fn space() -> impl Parser<()> {
    filter(|&c| c == ' ' || c == '\n')
        .ignored()
        .repeated()
        .map(|_| ())
}
