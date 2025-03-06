use chumsky::primitive::just;
use chumsky::Parser as _;
use sappho_primval::PrimVal;
use sappho_syntax_parsable::{Parsable, Parser};
use sappho_syntax_unparse::{Stream, Unparse};

use crate::{Base, Expr};

#[derive(Clone, Debug, Eq, PartialEq, derive_more::From)]
pub enum InnerExpr {
    Parens(Box<Expr>),
    Base(Base),
}

impl From<Expr> for InnerExpr {
    fn from(x: Expr) -> Self {
        Self::from(Box::new(x))
    }
}

impl From<PrimVal> for InnerExpr {
    fn from(pv: PrimVal) -> Self {
        Self::from(Base::from(pv))
    }
}

impl From<i64> for InnerExpr {
    fn from(i: i64) -> Self {
        Self::from(Base::from(i))
    }
}

impl Parsable for InnerExpr {
    fn parser() -> impl Parser<Self> {
        just('(')
            .map(|_| todo!("recursive parsable"))
            .or(Base::parser().map(InnerExpr::Base))
    }
}

impl Unparse for InnerExpr {
    fn unparse_into(&self, s: &mut Stream) {
        match self {
            InnerExpr::Parens(x) => x.unparse_into(s),
            InnerExpr::Base(x) => x.unparse_into(s),
        }
    }
}
