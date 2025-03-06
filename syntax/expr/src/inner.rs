use chumsky::primitive::just;
use chumsky::Parser as _;
use sappho_syntax_parsable::{Parsable, Parser};
use sappho_syntax_unparse::{Stream, Unparse};

use crate::{Base, Expr};

#[derive(Debug, derive_more::From)]
pub enum InnerExpr {
    Parens(Expr),
    BareBase(Base),
}

impl Parsable for InnerExpr {
    fn parser() -> impl Parser<Self> {
        just('(')
            .map(|_| todo!("recursive parsable"))
            .or(Base::parser().map(InnerExpr::BareBase))
    }
}

impl Unparse for InnerExpr {
    fn unparse_into(&self, s: &mut Stream) {
        match self {
            InnerExpr::Parens(x) => x.unparse_into(s),
            InnerExpr::BareBase(x) => x.unparse_into(s),
        }
    }
}
