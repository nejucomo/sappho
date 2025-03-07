use chumsky::recursive::recursive;
use chumsky::Parser as _;
use either::Either::{self, Left};
use sappho_primval::PrimVal;
use sappho_syntax_parsable::{Parsable, Parser, Recursive, RecursiveParsable};
use sappho_syntax_unparse::{Stream, Unparse};
use sappho_try_transform::TryTransformFrom;

use crate::{Applications, Base, InnerExpr, Lookups};

#[derive(Clone, Debug, Eq, PartialEq, derive_more::From)]
pub struct Expr(Applications);

impl Parsable for Expr {
    fn parser() -> impl Parser<Self> {
        recursive(Self::recursive_parser)
    }
}

impl RecursiveParsable<Expr> for Expr {
    fn recursive_parser(rec: Recursive<'_, Expr>) -> impl Parser<Self> {
        Applications::recursive_parser(rec).map(Expr::from)
    }
}

impl Unparse for Expr {
    fn unparse_into(&self, s: &mut Stream) {
        self.0.unparse_into(s)
    }
}

// Conversions
impl From<Expr> for Applications {
    fn from(value: Expr) -> Self {
        value.0
    }
}

impl TryTransformFrom<Expr> for Applications {
    fn try_transform_from(src: Expr) -> Either<Self, Expr> {
        Left(Self::from(src))
    }
}

impl From<Lookups> for Expr {
    fn from(value: Lookups) -> Self {
        Self::from(Applications::from(value))
    }
}

impl From<Expr> for Lookups {
    fn from(value: Expr) -> Self {
        Self::from(Applications::from(value))
    }
}

impl TryTransformFrom<Expr> for Lookups {
    fn try_transform_from(src: Expr) -> Either<Self, Expr> {
        Left(Self::from(src))
    }
}

impl From<InnerExpr> for Expr {
    fn from(value: InnerExpr) -> Self {
        Self::from(Applications::from(value))
    }
}

impl TryTransformFrom<Expr> for InnerExpr {
    fn try_transform_from(src: Expr) -> Either<Self, Expr> {
        Self::try_transform_from_via::<Lookups>(src)
    }
}

impl From<Base> for Expr {
    fn from(value: Base) -> Self {
        Self::from(Applications::from(value))
    }
}

impl TryTransformFrom<Expr> for Base {
    fn try_transform_from(src: Expr) -> Either<Self, Expr> {
        Self::try_transform_from_via::<Applications>(src)
    }
}

impl From<PrimVal> for Expr {
    fn from(value: PrimVal) -> Self {
        Self::from(Applications::from(value))
    }
}

impl TryTransformFrom<Expr> for PrimVal {
    fn try_transform_from(src: Expr) -> Either<Self, Expr> {
        Self::try_transform_from_via::<Base>(src)
    }
}

impl From<i32> for Expr {
    fn from(value: i32) -> Self {
        Self::from(Applications::from(value))
    }
}

impl TryTransformFrom<Expr> for i32 {
    fn try_transform_from(src: Expr) -> Either<Self, Expr> {
        Self::try_transform_from_via::<Base>(src)
    }
}
