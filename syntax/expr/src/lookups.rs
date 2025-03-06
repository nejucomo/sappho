use chumsky::prelude::just;
use chumsky::Parser as _;
use sappho_primval::PrimVal;
use sappho_syntax_idstore::ArcId;
use sappho_syntax_parsable::{Parsable, Parser};
use sappho_syntax_unparse::{Stream, Unparse};

use crate::InnerExpr;

#[derive(Clone, Debug, Eq, PartialEq, derive_more::From)]
pub struct Lookups {
    inner: InnerExpr,
    lookups: Vec<ArcId>,
}

impl Parsable for Lookups {
    fn parser() -> impl Parser<Self> {
        InnerExpr::parser()
            .then(just('.').ignore_then(ArcId::parser()).repeated())
            .map(Lookups::from)
    }
}

impl Unparse for Lookups {
    fn unparse_into(&self, s: &mut Stream) {
        s.write(&self.inner);
        for lookup in &self.lookups {
            s.write(".");
            s.write(lookup);
        }
    }
}

// From tree:
impl From<InnerExpr> for Lookups {
    fn from(inner: InnerExpr) -> Self {
        Lookups::from((inner, vec![]))
    }
}

impl From<PrimVal> for Lookups {
    fn from(pv: PrimVal) -> Self {
        Lookups::from(InnerExpr::from(pv))
    }
}

impl From<i64> for Lookups {
    fn from(i: i64) -> Self {
        Lookups::from(InnerExpr::from(i))
    }
}
