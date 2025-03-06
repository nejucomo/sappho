use chumsky::Parser as _;
use sappho_primval::PrimVal;
use sappho_syntax_idstore::ArcId;
use sappho_syntax_parsable::{Parsable, Parser};
use sappho_syntax_unparse::{Stream, Unparse};

#[derive(Clone, Debug, Eq, PartialEq, derive_more::From)]
pub enum Base {
    PrimVal(PrimVal),
    Deref(ArcId),
}

impl From<i64> for Base {
    fn from(i: i64) -> Self {
        Base::PrimVal(PrimVal::from(i))
    }
}

impl Parsable for Base {
    fn parser() -> impl Parser<Self> {
        PrimVal::parser()
            .map(Base::PrimVal)
            .or(ArcId::parser().map(Base::Deref))
    }
}

impl Unparse for Base {
    fn unparse_into(&self, s: &mut Stream) {
        match self {
            Base::PrimVal(x) => x.unparse_into(s),
            Base::Deref(x) => x.unparse_into(s),
        }
    }
}
