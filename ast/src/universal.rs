//! Subgrammars that are identical in both surface and eval ASTs
use crate::Identifier;

pub type Pattern = Identifier;

#[derive(Debug, PartialEq)]
pub enum UniversalExpr {
    Lit(Literal),
    Ref(Identifier),
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Num(f64),
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
