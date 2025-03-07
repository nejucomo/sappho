use chumsky::recursive::recursive;
use chumsky::Parser as _;
use sappho_syntax_leftassoc::LeftAssoc;
use sappho_syntax_parsable::{Parser, Recursive, RecursiveParsable};
use sappho_syntax_unparse::{Stream, Unparse};

use crate::{AttrLookup, InnerExpr};

#[derive(Clone, Debug, Eq, PartialEq, derive_more::From)]
pub struct Expr(Applications);

pub type Applications = LeftAssoc<Lookups, InnerExpr>;

pub type Lookups = LeftAssoc<InnerExpr, AttrLookup>;

impl Expr {
    pub fn parser() -> impl Parser<Self> {
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
    use sappho_try_transform::{TryTransformFrom, TryTransformInto};

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

    impl TryTransformInto<InnerExpr> for Expr {
        fn try_transform_into(self) -> Either<InnerExpr, Self> {
            self.0
                .try_transform_into()
                .left_and_then(|l: Lookups| l.try_transform_into().map_right(Applications::from))
                .map_right(Expr::from)
        }
    }

    impl TryTransformInto<i32> for Expr {
        fn try_transform_into(self) -> Either<i32, Self> {
            i32::try_transform_from_via::<InnerExpr>(self)
        }
    }
}
