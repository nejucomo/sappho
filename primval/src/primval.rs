use std::str::FromStr;

use chumsky::primitive::filter;
use chumsky::{text, Parser as _};
use sappho_parsable::error::ChumskyError;
use sappho_parsable::{Parsable, Parser};
use sappho_unparse::Unparse;

use self::PrimVal::*;

/// A [PrimVal] is a value the language inherently provides which excludes containing other values or value-references
///
/// Note that some [PrimVal] values _can_ be containers of [PrimVal] types. For example, a string contains chars, and a char is also a [PrimVal].
#[derive(Copy, Clone, Debug, PartialEq, derive_more::From)]
pub enum PrimVal {
    Num(Num),
    // TODO:
    //Bool(bool),
    //Char(char),

    // Less certain. Here we start heap allocations and lose [Copy]. We might want a distinction?
    // However, we do not need GC-coverage beyond rust [Box]/[Rc]/[Arc].
    //String(String),

    // Even less certain. Here is a partial-heterogenous container but it still excludes non-heap GC needs.
    //PrimArray(Vec<PrimVal>) ?
    // Or alternatively, homogenous containers?
    //ByteArray(Vec<u8>)
    //U32Array(Vec<u32>)
    // ... etc?
}

/// A number value
///
/// # TODO
///
/// ints (including bytes), big ints, decimals...
pub type Num = f64;

impl Parsable for PrimVal {
    fn parser() -> impl Parser<Self> {
        number().map(Num)
    }
}

impl Unparse for PrimVal {
    fn unparse_into(&self, s: &mut sappho_unparse::Stream) {
        match self {
            Num(n) => s.write(&n.to_string()),
        }
    }
}

fn number() -> impl Parser<f64> {
    let disallowed_trailing_char = filter(|&c: &char| c.is_alphabetic() || c.is_control())
        .try_map(|c, span| -> Result<(), ChumskyError> {
            Err(ChumskyError::custom(
                span,
                format!("unexpected {:?} in numeric literal", c),
            ))
        })
        .or_not();

    text::digits(10)
        .then_ignore(disallowed_trailing_char)
        .try_map(|digs: String, span| {
            f64::from_str(&digs).map_err(|e| ChumskyError::custom(span, e.to_string()))
        })
        .labelled("number")
}
