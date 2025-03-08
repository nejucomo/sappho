use chumsky::prelude::just;
use chumsky::Parser as _;
use either::Either::{self, Left, Right};
use sappho_syntax_listform::ListForm;
use sappho_syntax_parsable::error::ChumskyError;
use sappho_syntax_parsable::primitive::bracketed;
use sappho_syntax_parsable::{ParsableWith, Parser, Recursive};
use sappho_syntax_unparse::{Stream, Unparse};

use crate::Expr;

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct ListExpr(ListForm<Expr, Box<Expr>>);

impl<L> From<L> for ListExpr
where
    ListForm<Expr, Box<Expr>>: From<L>,
{
    fn from(value: L) -> Self {
        ListExpr(ListForm::from(value))
    }
}

impl ListExpr {
    /// Try to construct from a `Left(elem)` or `Right(tail)` iter
    ///
    /// # Errors
    ///
    /// The first such item to follow a tail is the `Err` case. There can be 0 or 1 tails.
    pub fn try_from_iter<I>(iter: I) -> Result<Self, Either<Expr, Expr>>
    where
        I: IntoIterator<Item = Either<Expr, Expr>>,
    {
        ListForm::try_from_iter(iter.into_iter().map(|ei| ei.map_right(Box::new)))
            .map_err(|ei| ei.map_right(|bx| *bx))
            .map(ListExpr)
    }
}

impl ParsableWith<Recursive<'_, Expr>> for ListExpr {
    fn make_parser_with(expr: Recursive<'_, Expr>) -> impl Parser<Self> {
        bracketed(
            ['[', ']'],
            (just("..").ignore_then(expr.clone()).map(Right))
                .or(expr
                    .clone()
                    .map(Left)
                    // Allow space before commas:
                    .then_opt_space())
                .separated_by(just(',').then_opt_space()),
        )
        .try_map(|v, span| {
            Self::try_from_iter(v).map_err(|xort| {
                ChumskyError::custom(span, format!("unexpected item after `..<tail>`: {xort:?}"))
            })
        })
    }
}

impl Unparse for ListExpr {
    fn unparse_into(&self, s: &mut Stream) {
        self.0.unparse_into(s)
    }
}

#[cfg(test)]
mod test_conversions {
    use either::Either;
    use sappho_try_transform::TryTransformInto;

    use crate::{Expr, ListExpr};

    impl TryTransformInto<Vec<Expr>> for ListExpr {
        fn try_transform_into(self) -> Either<Vec<Expr>, Self> {
            self.0.try_transform_into().map_right(ListExpr::from)
        }
    }
}
