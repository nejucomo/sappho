use chumsky::Parser as _;
use sappho_primval::PrimVal;
use sappho_syntax_parsable::{Parsable, Parser};
use sappho_syntax_unparse::{Stream, Unparse};

use crate::{InnerExpr, Lookups};

#[derive(Clone, Debug, Eq, PartialEq, derive_more::From)]
pub struct Applications {
    lookups: Lookups,
    args: Vec<InnerExpr>,
}

impl Parsable for Applications {
    fn parser() -> impl Parser<Self> {
        Lookups::parser()
            .then(InnerExpr::parser().repeated())
            .map(Applications::from)
    }
}

impl Unparse for Applications {
    fn unparse_into(&self, s: &mut Stream) {
        s.write(&self.lookups);
        for arg in &self.args {
            s.write(" ");
            s.write(arg);
        }
    }
}

// From tree:
impl From<Lookups> for Applications {
    fn from(v: Lookups) -> Self {
        Self::from((v, vec![]))
    }
}

impl From<InnerExpr> for Applications {
    fn from(v: InnerExpr) -> Self {
        Self::from(Lookups::from(v))
    }
}

impl From<PrimVal> for Applications {
    fn from(v: PrimVal) -> Self {
        Self::from(Lookups::from(v))
    }
}

impl From<i64> for Applications {
    fn from(v: i64) -> Self {
        Self::from(Lookups::from(v))
    }
}
