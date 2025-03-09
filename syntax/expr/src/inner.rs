use chumsky::primitive::just;
use chumsky::Parser as _;
use sappho_syntax_idstore::ArcId;
use sappho_syntax_parsable::{Parsable, ParsableWith, Parser, Recursive};
use sappho_syntax_universal::Unx;
use sappho_syntax_unparse::{Stream, Unparse};

use crate::{Expr, ListExpr};

#[derive(Clone, Debug, Eq, PartialEq, derive_more::From)]
pub enum InnerExpr {
    #[from(Unx, i32, ArcId)]
    Uni(Unx),
    #[from]
    Parens(Box<Expr>),
    #[from(ListExpr, Vec<Expr>)]
    ListExpr(ListExpr),
}

impl<'r> ParsableWith<Recursive<'r, Expr>> for InnerExpr {
    fn make_parser_with(rec: Recursive<'r, Expr>) -> impl Parser<Self> {
        Unx::parser()
            .map(InnerExpr::from)
            .or(ListExpr::parser_with(rec.clone()).map(InnerExpr::from))
            .or(just('(')
                .ignore_then(rec)
                .then_ignore(just(')'))
                .map(Box::from)
                .map(InnerExpr::from))
    }
}

impl Unparse for InnerExpr {
    fn unparse_into(&self, s: &mut Stream) {
        match self {
            InnerExpr::Uni(x) => x.unparse_into(s),
            InnerExpr::Parens(x) => x.unparse_into(s),
            InnerExpr::ListExpr(x) => x.unparse_into(s),
        }
    }
}

#[cfg(test)]
mod test_conversions {
    use either::Either::{self, Left, Right};
    use sappho_syntax_idstore::ArcId;
    use sappho_syntax_universal::Unx;
    use sappho_try_transform::{TryTransformFrom, TryTransformInto};

    use crate::{Expr, InnerExpr, ListExpr};

    impl TryTransformInto<Unx> for InnerExpr {
        fn try_transform_into(self) -> Either<Unx, Self> {
            match self {
                InnerExpr::Uni(base) => Left(base),
                other => Right(other),
            }
        }
    }

    impl TryTransformInto<i32> for InnerExpr {
        fn try_transform_into(self) -> Either<i32, Self> {
            i32::try_transform_from_via::<Unx>(self)
        }
    }

    impl TryTransformInto<ArcId> for InnerExpr {
        fn try_transform_into(self) -> Either<ArcId, Self> {
            ArcId::try_transform_from_via::<Unx>(self)
        }
    }

    impl TryTransformInto<ListExpr> for InnerExpr {
        fn try_transform_into(self) -> Either<ListExpr, Self> {
            match self {
                InnerExpr::ListExpr(x) => Left(x),
                other => Right(other),
            }
        }
    }

    impl TryTransformInto<Vec<Expr>> for InnerExpr {
        fn try_transform_into(self) -> Either<Vec<Expr>, Self> {
            Vec::try_transform_from_via::<ListExpr>(self)
        }
    }
}
