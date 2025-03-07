use chumsky::prelude::just;
use chumsky::Parser as _;
use either::Either::{self, Left};
use sappho_primval::PrimVal;
use sappho_syntax_idstore::ArcId;
use sappho_syntax_parsable::{Parsable, Parser, Recursive, RecursiveParsable};
use sappho_syntax_unparse::{Stream, Unparse};
use sappho_try_transform::TryTransformFrom;

use crate::{Base, Expr, InnerExpr};

#[derive(Clone, Debug, Eq, PartialEq, derive_more::From)]
pub struct Lookups {
    inner: InnerExpr,
    lookups: Vec<ArcId>,
}

impl RecursiveParsable<Expr> for Lookups {
    fn recursive_parser(rec: Recursive<'_, Expr>) -> impl Parser<Self> {
        InnerExpr::recursive_parser(rec)
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

// Conversions:
impl From<InnerExpr> for Lookups {
    fn from(inner: InnerExpr) -> Self {
        Lookups::from((inner, vec![]))
    }
}
impl From<Lookups> for InnerExpr {
    fn from(value: Lookups) -> Self {
        value.inner
    }
}

impl TryTransformFrom<Lookups> for InnerExpr {
    fn try_transform_from(src: Lookups) -> Either<Self, Lookups> {
        Left(InnerExpr::from(src))
    }
}

impl From<Base> for Lookups {
    fn from(value: Base) -> Self {
        Self::from(InnerExpr::from(value))
    }
}

impl TryTransformFrom<Lookups> for Base {
    fn try_transform_from(src: Lookups) -> Either<Self, Lookups> {
        Self::try_transform_from_via::<InnerExpr>(src)
    }
}

impl From<PrimVal> for Lookups {
    fn from(pv: PrimVal) -> Self {
        Lookups::from(InnerExpr::from(pv))
    }
}

impl TryTransformFrom<Lookups> for PrimVal {
    fn try_transform_from(src: Lookups) -> Either<Self, Lookups> {
        Self::try_transform_from_via::<InnerExpr>(src)
    }
}

impl From<i32> for Lookups {
    fn from(i: i32) -> Self {
        Lookups::from(InnerExpr::from(i))
    }
}

impl TryTransformFrom<Lookups> for i32 {
    fn try_transform_from(src: Lookups) -> Either<Self, Lookups> {
        Self::try_transform_from_via::<InnerExpr>(src)
    }
}
