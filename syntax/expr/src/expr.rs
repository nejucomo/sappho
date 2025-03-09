use chumsky::recursive::recursive;
use chumsky::Parser as _;
use sappho_syntax_idstore::ArcId;
use sappho_syntax_parsable::{ParsableWith, Parser, Recursive};
use sappho_syntax_unparse::{Stream, Unparse};

use crate::{Applications, InnerExpr, ListExpr};

#[derive(Clone, Debug, Eq, PartialEq, derive_more::From, derive_more::Into)]
#[from(Applications, InnerExpr, i32, ArcId, ListExpr, Vec<Expr>)]
pub struct Expr(Applications);

impl Expr {
    pub fn parser() -> impl Parser<Self> {
        recursive(Self::parser_with)
    }
}

impl<'r> ParsableWith<Recursive<'r, Expr>> for Expr {
    fn make_parser_with(rec: Recursive<'r, Expr>) -> impl Parser<Self> {
        Applications::parser_with(rec).then_space().map(Expr::from)
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
    use sappho_syntax_idstore::ArcId;
    use sappho_try_transform::{TryTransformFrom, TryTransformInto};

    use crate::{Applications, Expr, InnerExpr, ListExpr, Lookups};

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

    impl TryTransformInto<ArcId> for Expr {
        fn try_transform_into(self) -> Either<ArcId, Self> {
            ArcId::try_transform_from_via::<InnerExpr>(self)
        }
    }

    impl TryTransformInto<ListExpr> for Expr {
        fn try_transform_into(self) -> Either<ListExpr, Self> {
            ListExpr::try_transform_from_via::<InnerExpr>(self)
        }
    }

    impl TryTransformInto<Vec<Expr>> for Expr {
        fn try_transform_into(self) -> Either<Vec<Expr>, Self> {
            Vec::try_transform_from_via::<InnerExpr>(self)
        }
    }
}
