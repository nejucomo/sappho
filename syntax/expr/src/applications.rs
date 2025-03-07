use chumsky::Parser as _;
use either::Either::{self, Left};
use sappho_primval::PrimVal;
use sappho_syntax_parsable::{Parser, Recursive, RecursiveParsable};
use sappho_syntax_unparse::{Stream, Unparse};
use sappho_try_transform::TryTransformFrom;

use crate::{Base, Expr, InnerExpr, Lookups};

#[derive(Clone, Debug, Eq, PartialEq, derive_more::From)]
pub struct Applications {
    lookups: Lookups,
    args: Vec<InnerExpr>,
}

impl RecursiveParsable<Expr> for Applications {
    fn recursive_parser(rec: Recursive<'_, Expr>) -> impl Parser<Self> {
        Lookups::recursive_parser(rec.clone())
            .then(InnerExpr::recursive_parser(rec).repeated())
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

// Conversions
impl From<Lookups> for Applications {
    fn from(v: Lookups) -> Self {
        Self::from((v, vec![]))
    }
}

impl From<Applications> for Lookups {
    fn from(value: Applications) -> Self {
        value.lookups
    }
}

impl TryTransformFrom<Applications> for Lookups {
    fn try_transform_from(src: Applications) -> Either<Self, Applications> {
        Left(Self::from(src))
    }
}

impl From<InnerExpr> for Applications {
    fn from(v: InnerExpr) -> Self {
        Self::from(Lookups::from(v))
    }
}

impl TryTransformFrom<Applications> for InnerExpr {
    fn try_transform_from(src: Applications) -> Either<Self, Applications> {
        Self::try_transform_from_via::<Lookups>(src)
    }
}

impl From<Base> for Applications {
    fn from(value: Base) -> Self {
        Self::from(Lookups::from(value))
    }
}

impl TryTransformFrom<Applications> for Base {
    fn try_transform_from(src: Applications) -> Either<Self, Applications> {
        Self::try_transform_from_via::<Lookups>(src)
    }
}

impl From<PrimVal> for Applications {
    fn from(v: PrimVal) -> Self {
        Self::from(Lookups::from(v))
    }
}

impl TryTransformFrom<Applications> for PrimVal {
    fn try_transform_from(src: Applications) -> Either<Self, Applications> {
        Self::try_transform_from_via::<Lookups>(src)
    }
}

impl From<i32> for Applications {
    fn from(v: i32) -> Self {
        Self::from(Lookups::from(v))
    }
}

impl TryTransformFrom<Applications> for i32 {
    fn try_transform_from(src: Applications) -> Either<Self, Applications> {
        Self::try_transform_from_via::<Lookups>(src)
    }
}
