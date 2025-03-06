use sappho_syntax_parsable::{Parsable, Parser};
use sappho_syntax_unparse::{Stream, Unparse};

use crate::parseutil::number;

use self::PrimVal::*;

/// A primitive value, such as `3.1415`
///
/// # Properties
///
/// Primvals have these language properties:
///
/// - `Copy`-able
/// - Runtime values
/// - Immutable
/// - Literal syntax
#[derive(Copy, Clone, Debug, Eq, PartialEq, derive_more::From)]
pub enum PrimVal {
    /// A literal number value, such as `42`.
    Num(i64),
}

impl Parsable for PrimVal {
    fn parser() -> impl Parser<Self> {
        use chumsky::Parser as _;

        number().map(Num).labelled("number")
    }
}

impl Unparse for PrimVal {
    fn unparse_into(&self, s: &mut Stream) {
        match self {
            Num(x) => s.write(&x.to_string()),
        }
    }
}
