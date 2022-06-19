//! Subgrammars that are identical in both surface and eval ASTs

mod lit;

use crate::Identifier;
use std::fmt;

pub use self::lit::Literal;

pub type Pattern = Identifier;

#[derive(Debug, PartialEq)]
pub enum UniversalExpr {
    Lit(Literal),
    Ref(Identifier),
}

impl From<f64> for UniversalExpr {
    fn from(f: f64) -> UniversalExpr {
        UniversalExpr::Lit(Literal::Num(f))
    }
}

impl From<Identifier> for UniversalExpr {
    fn from(ident: Identifier) -> UniversalExpr {
        UniversalExpr::Ref(ident)
    }
}

impl fmt::Display for UniversalExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use UniversalExpr::*;

        match self {
            Lit(x) => x.fmt(f),
            Ref(x) => x.fmt(f),
        }
    }
}
