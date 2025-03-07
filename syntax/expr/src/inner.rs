use chumsky::primitive::just;
use chumsky::Parser as _;
use sappho_syntax_parsable::{Parsable, ParsableWith, Parser, Recursive};
use sappho_syntax_unparse::{Stream, Unparse};

use crate::{Base, Expr};

#[derive(Clone, Debug, Eq, PartialEq, derive_more::From)]
pub enum InnerExpr {
    Parens(Box<Expr>),
    Base(Base),
}

impl From<Expr> for InnerExpr {
    fn from(value: Expr) -> Self {
        InnerExpr::Parens(Box::new(value))
    }
}

impl<'r> ParsableWith<Recursive<'r, Expr>> for InnerExpr {
    fn parser_with(recurse: Recursive<'r, Expr>) -> impl Parser<Self> {
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

#[cfg(test)]
mod test_conversions {
    use either::Either::{self, Left, Right};
    use sappho_syntax_idstore::ArcId;
    use sappho_try_transform::{TryTransformFrom, TryTransformInto};

    use crate::{Base, InnerExpr};

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
}
