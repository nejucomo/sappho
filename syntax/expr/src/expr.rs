use chumsky::recursive::recursive;
use chumsky::Parser as _;
use sappho_syntax_leftassoc::LeftAssoc;
use sappho_syntax_parsable::{ParsableWith, Parser, Recursive};
use sappho_syntax_unparse::{Stream, Unparse};

use crate::{AttrLookup, InnerExpr};

#[derive(Clone, Debug, Eq, PartialEq, derive_more::From, derive_more::Into)]
pub struct Expr(Applications);

pub type Applications = LeftAssoc<Lookups, InnerExpr>;

pub type Lookups = LeftAssoc<InnerExpr, AttrLookup>;

impl Expr {
    pub fn parser() -> impl Parser<Self> {
        recursive(Self::parser_with)
    }
}

impl<'r> ParsableWith<Recursive<'r, Expr>> for Expr {
    fn make_parser_with(rec: Recursive<'r, Expr>) -> impl Parser<Self> {
        Applications::parser_with(((rec.clone(), ()), rec))
            .then_space()
            .map(Expr::from)
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
    use sappho_syntax_idstore::ArcId;
    use sappho_try_transform::{TryTransformFrom, TryTransformInto};

    use crate::{Applications, Base, Expr, InnerExpr, ListExpr, Lookups};

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

    impl From<i32> for Expr {
        fn from(value: i32) -> Self {
            Expr::from(InnerExpr::Base(Base::PrimVal(PrimVal::from(value))))
        }
    }

    impl TryTransformInto<i32> for Expr {
        fn try_transform_into(self) -> Either<i32, Self> {
            i32::try_transform_from_via::<InnerExpr>(self)
        }
    }

    impl From<ArcId> for Expr {
        fn from(value: ArcId) -> Self {
            Expr::from(InnerExpr::from(value))
        }
    }

    impl TryTransformInto<ArcId> for Expr {
        fn try_transform_into(self) -> Either<ArcId, Self> {
            ArcId::try_transform_from_via::<InnerExpr>(self)
        }
    }

    impl From<ListExpr> for Expr {
        fn from(value: ListExpr) -> Self {
            Expr::from(InnerExpr::from(value))
        }
    }

    impl TryTransformInto<ListExpr> for Expr {
        fn try_transform_into(self) -> Either<ListExpr, Self> {
            ListExpr::try_transform_from_via::<InnerExpr>(self)
        }
    }

    impl From<Vec<Expr>> for Expr {
        fn from(value: Vec<Expr>) -> Self {
            Expr::from(InnerExpr::from(value))
        }
    }

    impl TryTransformInto<Vec<Expr>> for Expr {
        fn try_transform_into(self) -> Either<Vec<Expr>, Self> {
            Vec::try_transform_from_via::<InnerExpr>(self)
        }
    }
}
