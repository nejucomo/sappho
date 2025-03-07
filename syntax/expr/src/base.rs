use chumsky::Parser as _;
use either::Either::{self, Left, Right};
use sappho_primval::PrimVal;
use sappho_syntax_idstore::ArcId;
use sappho_syntax_parsable::{Parsable, Parser};
use sappho_syntax_unparse::{Stream, Unparse};
use sappho_try_transform::{TryTransformFrom, TryTransformInto};

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

impl TryTransformInto<PrimVal> for Base {
    fn try_transform_into(self) -> Either<PrimVal, Self> {
        match self {
            Base::PrimVal(x) => Left(x),
            other => Right(other),
        }
    }
}

impl TryTransformInto<i32> for Base {
    fn try_transform_into(self) -> Either<i32, Self> {
        i32::try_transform_from_via::<PrimVal>(self)
    }
}

impl TryTransformInto<ArcId> for Base {
    fn try_transform_into(self) -> Either<ArcId, Self> {
        match self {
            Base::Deref(x) => Left(x),
            other => Right(other),
        }
    }
}
