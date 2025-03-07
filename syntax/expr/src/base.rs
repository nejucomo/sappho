use chumsky::Parser as _;
use either::Either::{self, Left, Right};
use sappho_primval::PrimVal;
use sappho_syntax_idstore::ArcId;
use sappho_syntax_parsable::{Parsable, Parser};
use sappho_syntax_unparse::{Stream, Unparse};
use sappho_try_transform::TryTransformFrom;

#[derive(Clone, Debug, Eq, PartialEq, derive_more::From)]
pub enum Base {
    PrimVal(PrimVal),
    Deref(ArcId),
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

// Conversions
impl From<i32> for Base {
    fn from(i: i32) -> Self {
        Base::PrimVal(PrimVal::from(i))
    }
}

impl TryTransformFrom<Base> for PrimVal {
    fn try_transform_from(src: Base) -> Either<Self, Base> {
        match src {
            Base::PrimVal(x) => Left(x),
            other => Right(other),
        }
    }
}

impl TryTransformFrom<Base> for i32 {
    fn try_transform_from(src: Base) -> Either<Self, Base> {
        i32::try_transform_from_via::<PrimVal>(src)
    }
}

impl TryTransformFrom<Base> for ArcId {
    fn try_transform_from(src: Base) -> Either<Self, Base> {
        match src {
            Base::Deref(x) => Left(x),
            other => Right(other),
        }
    }
}
