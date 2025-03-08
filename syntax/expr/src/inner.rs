use chumsky::primitive::just;
use chumsky::Parser as _;
use sappho_syntax_parsable::{Parsable, ParsableWith, Parser, Recursive};
use sappho_syntax_unparse::{Stream, Unparse};

use crate::{Base, Expr, ListExpr};

#[derive(Clone, Debug, Eq, PartialEq, derive_more::From)]
pub enum InnerExpr {
    Base(Base),
    Parens(Box<Expr>),
    ListExpr(ListExpr),
}

impl From<Expr> for InnerExpr {
    fn from(value: Expr) -> Self {
        InnerExpr::Parens(Box::new(value))
    }
}

impl<'r> ParsableWith<Recursive<'r, Expr>> for InnerExpr {
    fn make_parser_with(rec: Recursive<'r, Expr>) -> impl Parser<Self> {
        Base::parser()
            .map(InnerExpr::from)
            .or(ListExpr::parser_with(rec.clone()).map(InnerExpr::from))
            .or(just('(')
                .ignore_then(rec)
                .then_ignore(just(')'))
                .map(InnerExpr::from))
    }
}

impl Unparse for InnerExpr {
    fn unparse_into(&self, s: &mut Stream) {
        match self {
            InnerExpr::Base(x) => x.unparse_into(s),
            InnerExpr::Parens(x) => x.unparse_into(s),
            InnerExpr::ListExpr(x) => x.unparse_into(s),
        }
    }
}

#[cfg(test)]
mod test_conversions {
    use either::Either::{self, Left, Right};
    use sappho_syntax_idstore::ArcId;
    use sappho_try_transform::{TryTransformFrom, TryTransformInto};

    use crate::{Base, Expr, InnerExpr, ListExpr};

    impl TryTransformInto<Base> for InnerExpr {
        fn try_transform_into(self) -> Either<Base, Self> {
            match self {
                InnerExpr::Base(base) => Left(base),
                other => Right(other),
            }
        }
    }

    impl From<i32> for InnerExpr {
        fn from(value: i32) -> Self {
            Self::from(Base::from(value))
        }
    }

    impl TryTransformInto<i32> for InnerExpr {
        fn try_transform_into(self) -> Either<i32, Self> {
            i32::try_transform_from_via::<Base>(self)
        }
    }

    impl From<ArcId> for InnerExpr {
        fn from(value: ArcId) -> Self {
            Self::from(Base::from(value))
        }
    }

    impl TryTransformInto<ArcId> for InnerExpr {
        fn try_transform_into(self) -> Either<ArcId, Self> {
            ArcId::try_transform_from_via::<Base>(self)
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

    impl From<Vec<Expr>> for InnerExpr {
        fn from(value: Vec<Expr>) -> Self {
            Self::from(ListExpr::from(value))
        }
    }

    impl TryTransformInto<Vec<Expr>> for InnerExpr {
        fn try_transform_into(self) -> Either<Vec<Expr>, Self> {
            Vec::try_transform_from_via::<ListExpr>(self)
        }
    }
}
