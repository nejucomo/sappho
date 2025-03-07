use chumsky::primitive::just;
use chumsky::Parser as _;
use either::Either::{self, Left, Right};
use sappho_primval::PrimVal;
use sappho_syntax_idstore::ArcId;
use sappho_syntax_parsable::{Parsable, Parser, Recursive, RecursiveParsable};
use sappho_syntax_unparse::{Stream, Unparse};
use sappho_try_transform::TryTransformFrom;

use crate::{Base, Expr};

#[derive(Clone, Debug, Eq, PartialEq, derive_more::From)]
pub enum InnerExpr {
    Parens(Box<Expr>),
    Base(Base),
}

impl RecursiveParsable<Expr> for InnerExpr {
    fn recursive_parser(recurse: Recursive<'_, Expr>) -> impl Parser<Self> {
        Base::parser().map(InnerExpr::from).or(just('(')
            .ignore_then(recurse)
            .then_ignore(just(')'))
            .map(InnerExpr::from))
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

// Conversions
impl From<Expr> for InnerExpr {
    fn from(x: Expr) -> Self {
        Self::from(Box::new(x))
    }
}

impl From<ArcId> for InnerExpr {
    fn from(value: ArcId) -> Self {
        Self::from(Base::from(value))
    }
}

impl From<PrimVal> for InnerExpr {
    fn from(pv: PrimVal) -> Self {
        Self::from(Base::from(pv))
    }
}

impl From<i32> for InnerExpr {
    fn from(i: i32) -> Self {
        Self::from(Base::from(i))
    }
}

impl TryTransformFrom<InnerExpr> for Expr {
    fn try_transform_from(src: InnerExpr) -> Either<Self, InnerExpr> {
        match src {
            InnerExpr::Parens(x) => Left(*x),
            other => Right(other),
        }
    }
}

impl TryTransformFrom<InnerExpr> for Base {
    fn try_transform_from(src: InnerExpr) -> Either<Self, InnerExpr> {
        match src {
            InnerExpr::Base(x) => Left(x),
            other => Right(other),
        }
    }
}

impl TryTransformFrom<InnerExpr> for ArcId {
    fn try_transform_from(src: InnerExpr) -> Either<Self, InnerExpr> {
        Self::try_transform_from_via::<Base>(src)
    }
}

impl TryTransformFrom<InnerExpr> for PrimVal {
    fn try_transform_from(src: InnerExpr) -> Either<Self, InnerExpr> {
        Self::try_transform_from_via::<Base>(src)
    }
}

impl TryTransformFrom<InnerExpr> for i32 {
    fn try_transform_from(src: InnerExpr) -> Either<Self, InnerExpr> {
        Self::try_transform_from_via::<PrimVal>(src)
    }
}
