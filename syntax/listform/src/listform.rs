use chumsky::primitive::just;
use chumsky::Parser as _;
use either::Either::{self, Left, Right};
use sappho_syntax_parsable::error::ChumskyError;
use sappho_syntax_parsable::primitive::bracketed;
use sappho_syntax_parsable::{ParsableWith, Parser};
use sappho_syntax_unparse::{Stream, Unparse};
use sappho_try_transform::TryTransformInto;

use std::fmt;

use crate::lfg::ListFormGeneric;
use crate::ListFormIter;

/// A general structure for a sequence of items, with an optional tail, used for both list patterns
/// and expressions in the ast, examples: `[]`, `[32]`, `[a, b, ..t]`
#[derive(Clone, Debug, Eq, PartialEq, derive_more::From, derive_more::Into)]
pub struct ListForm<Elem, Tail>(ListFormGeneric<Vec<Elem>, Tail>);

impl<X, T> ListForm<X, T> {
    pub fn new<I>(body: I, tail: Option<T>) -> Self
    where
        I: IntoIterator<Item = X>,
    {
        ListForm(ListFormGeneric::new(body.into_iter().collect(), tail))
    }

    /// Try to construct from a `Left(elem)` or `Right(tail)` iter
    ///
    /// # Errors
    ///
    /// The first such item to follow a tail is the `Err` case. There can be 0 or 1 tails.
    pub fn try_from_iter<I>(iter: I) -> Result<Self, Either<X, T>>
    where
        I: IntoIterator<Item = Either<X, T>>,
    {
        ListFormGeneric::try_from_iter(iter).map(ListForm)
    }

    pub fn is_empty(&self) -> bool {
        self.0.xs.is_empty() && self.0.optail.is_none()
    }

    pub fn prepend(mut self, head: X) -> Self {
        self.0.xs.insert(0, head);
        self
    }
}

impl<X, T> ParsableWith<()> for ListForm<X, T>
where
    X: ParsableWith<()>,
    T: ParsableWith<()>,
{
    fn make_parser_with(_: ()) -> impl Parser<Self> {
        Self::parser_with(((), ()))
    }
}

impl<X, XP, T, TP> ParsableWith<(XP, TP)> for ListForm<X, T>
where
    X: ParsableWith<XP>,
    T: ParsableWith<TP>,
{
    fn make_parser_with((xp, tp): (XP, TP)) -> impl Parser<Self> {
        bracketed(
            ['[', ']'],
            (just("..").ignore_then(T::parser_with(tp)).map(Right))
                .or(X::parser_with(xp)
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

impl<X, T> Unparse for ListForm<X, T>
where
    X: Unparse,
    T: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        use sappho_syntax_unparse::Brackets::Square;
        use sappho_syntax_unparse::Break::OptSpace;

        if self.is_empty() {
            s.write("[]")
        } else {
            s.bracketed(Square, |subs| {
                let mut sep = "";

                for xort in self.0.as_ref() {
                    subs.write(sep);
                    sep = ",";
                    subs.write(&OptSpace);

                    match xort {
                        Left(elem) => {
                            subs.write(elem);
                        }
                        Right(tail) => {
                            subs.write("..");
                            subs.write(tail);
                        }
                    }
                }
            });
        }
    }
}

impl<X, T> Default for ListForm<X, T> {
    fn default() -> Self {
        ListForm(ListFormGeneric::default())
    }
}

impl<X, T, E> ListForm<X, Result<T, E>> {
    pub fn transpose_tail(self) -> Result<ListForm<X, T>, E> {
        Ok(ListForm::new(self.0.xs, self.0.optail.transpose()?))
    }
}

impl<X, T> IntoIterator for ListForm<X, T> {
    type Item = Either<X, T>;
    type IntoIter = ListFormIter<std::vec::IntoIter<X>, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<X, T> TryFrom<Vec<Either<X, T>>> for ListForm<X, T> {
    type Error = Either<X, T>;

    fn try_from(value: Vec<Either<X, T>>) -> Result<Self, Self::Error> {
        Self::try_from_iter(value)
    }
}

impl<X, T> fmt::Display for ListForm<X, T>
where
    X: Unparse,
    T: Unparse,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unparse().fmt(f)
    }
}

// Transforms
impl<X, T> From<(Vec<X>, Option<T>)> for ListForm<X, T> {
    fn from((xs, opt): (Vec<X>, Option<T>)) -> Self {
        ListForm::new(xs, opt)
    }
}

impl<X, T> From<ListForm<X, T>> for (Vec<X>, Option<T>) {
    fn from(lf: ListForm<X, T>) -> Self {
        lf.0.into()
    }
}

impl<X, T> From<Vec<X>> for ListForm<X, T> {
    fn from(value: Vec<X>) -> Self {
        ListForm::from(ListFormGeneric::new(value, None))
    }
}

impl<X, T> TryTransformInto<Vec<X>> for ListForm<X, T> {
    fn try_transform_into(self) -> Either<Vec<X>, Self> {
        if self.0.optail.is_none() {
            Left(self.0.xs)
        } else {
            Right(self)
        }
    }
}
