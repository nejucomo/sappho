use chumsky::Parser as _;
use sappho_primval::PrimVal;
use sappho_syntax_parsable::{Parsable, Parser};
use sappho_syntax_unparse::{Stream, Unparse};

use crate::{Applications, InnerExpr, Lookups};

#[derive(Clone, Debug, Eq, PartialEq, derive_more::From)]
pub struct Expr(Applications);

impl Parsable for Expr {
    fn parser() -> impl Parser<Self> {
        Applications::parser().map(Expr::from)
    }
}

impl Unparse for Expr {
    fn unparse_into(&self, s: &mut Stream) {
        self.0.unparse_into(s)
    }
}

// From-tree:
impl From<Lookups> for Expr {
    fn from(value: Lookups) -> Self {
        Self::from(Applications::from(value))
    }
}

impl From<InnerExpr> for Expr {
    fn from(value: InnerExpr) -> Self {
        Self::from(Applications::from(value))
    }
}

impl From<PrimVal> for Expr {
    fn from(value: PrimVal) -> Self {
        Self::from(Applications::from(value))
    }
}

impl From<i64> for Expr {
    fn from(value: i64) -> Self {
        Self::from(Applications::from(value))
    }
}
