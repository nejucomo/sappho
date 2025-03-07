use chumsky::prelude::just;
use chumsky::Parser as _;
use sappho_syntax_idstore::ArcId;
use sappho_syntax_parsable::{Parsable, Parser, Recursive, RecursiveParsable};
use sappho_syntax_unparse::{Stream, Unparse};

use crate::{Expr, InnerExpr};

#[derive(Clone, Debug, Eq, PartialEq, derive_more::From)]
pub struct Lookups {
    inner: InnerExpr,
    lookups: Vec<ArcId>,
}

impl From<InnerExpr> for Lookups {
    fn from(inner: InnerExpr) -> Self {
        Lookups {
            inner,
            lookups: vec![],
        }
    }
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

#[cfg(test)]
mod test_conversions {
    use either::Either::{self, Left, Right};
    use sappho_try_transform::TryTransformFrom;

    use crate::{InnerExpr, Lookups};

    impl From<i32> for Lookups {
        fn from(value: i32) -> Self {
            Self::from(InnerExpr::from(value))
        }
    }

    impl TryTransformFrom<Lookups> for InnerExpr {
        fn try_transform_from(src: Lookups) -> Either<Self, Lookups> {
            if src.lookups.is_empty() {
                Left(src.inner)
            } else {
                Right(src)
            }
        }
    }

    impl TryTransformFrom<Lookups> for i32 {
        fn try_transform_from(src: Lookups) -> Either<Self, Lookups> {
            i32::try_transform_from_via::<InnerExpr>(src)
        }
    }
}
