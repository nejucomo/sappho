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
    chumsky::primitive::filter(|&c| c == ' ' || c == '\n')
        .ignored()
        .repeated()
        .map(|_| ())
}
