use chumsky::recursive::recursive;
use chumsky::Parser as _;
use sappho_syntax_parsable::{Parsable, Parser, Recursive, RecursiveParsable};
use sappho_syntax_unparse::{Stream, Unparse};

use crate::Applications;

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

#[cfg(test)]
mod test_conversions {
    use either::Either;
    use sappho_primval::PrimVal;
    use sappho_try_transform::TryTransformFrom;

    use crate::{Applications, Base, Expr, InnerExpr, Lookups};

    impl From<i32> for Expr {
        fn from(value: i32) -> Self {
            Expr::from(InnerExpr::Base(Base::PrimVal(PrimVal::from(value))))
        }
    }

    impl From<InnerExpr> for Expr {
        fn from(value: InnerExpr) -> Self {
            Expr::from(Applications::from(Lookups::from(value)))
        }
    }

    impl TryTransformFrom<Expr> for i32 {
        fn try_transform_from(src: Expr) -> Either<Self, Expr> {
            Self::try_transform_from(src.0).map_right(Expr::from)
        }
    }
}
