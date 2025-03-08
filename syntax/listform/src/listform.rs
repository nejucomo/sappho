use chumsky::primitive::just;
use chumsky::recursive::recursive;
use chumsky::Parser as _;
use either::Either::{self, Left, Right};
use sappho_syntax_parsable::{ParsableWith, Parser, Recursive};
use sappho_syntax_unparse::{Stream, Unparse};
use sappho_try_transform::TryTransformInto;

use std::fmt;

use crate::lfg::ListFormGeneric;
use crate::ListFormIter;

/// A general structure for a sequence of items, with an optional tail, used for both list patterns
/// and expressions in the ast, examples: `[]`, `[32]`, `[a, b, ..t]`
#[derive(Clone, Debug, Eq, PartialEq, derive_more::From)]
pub struct ListForm<Elem, Tail>(ListFormGeneric<Vec<Elem>, Tail>);

impl<X, T> ListForm<X, T> {
    pub fn new<I>(body: I, tail: Option<T>) -> Self
    where
        I: IntoIterator<Item = X>,
    {
        ListForm(ListFormGeneric::new(body.into_iter().collect(), tail))
    }

    pub fn is_empty(&self) -> bool {
        self.0.xs.is_empty() && self.0.optail.is_none()
    }

    pub fn prepend(mut self, head: X) -> Self {
        self.0.xs.insert(0, head);
        self
    }
}

impl<X, XP, T, TP> ParsableWith<(XP, TP)> for ListForm<X, T>
where
    X: ParsableWith<XP> + 'static,
    T: ParsableWith<TP> + 'static,
    XP: Clone + 'static,
    TP: Clone + 'static,
{
    fn make_parser_with((xp, tp): (XP, TP)) -> impl Parser<Self> {
        just('[').then_opt_space().ignore_then(recursive(|rec| {
            parse_after_open_bracket::<X, XP, T, TP>(xp, tp, rec)
        }))
    }
}

fn parse_after_open_bracket<X, XP, T, TP>(
    xp: XP,
    tp: TP,
    rec: Recursive<ListForm<X, T>>,
) -> impl Parser<ListForm<X, T>> + '_
where
    X: ParsableWith<XP> + 'static,
    T: ParsableWith<TP> + 'static,
    XP: Clone + 'static,
    TP: Clone + 'static,
{
    let tail = just("..")
        .then_opt_space()
        .ignore_then(T::parser_with(tp.clone()))
        .or_not()
        .then_ignore(just(']'))
        .map(|optail| ListForm::new(vec![], optail));

    let elem = X::parser_with(xp.clone()).then_ignore(just(',').then_opt_space());

    let elem_and_rest = elem.then(rec).map(|(x, mut lf)| {
        lf.0.xs.push(x);
        lf
    });

    tail.or(elem_and_rest)
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

/// # Panic
///
/// This panics if a `Right` is ever encountered in any position besides the last element
impl<X, T> FromIterator<Either<X, T>> for ListForm<X, T>
where
    X: std::fmt::Debug,
    T: std::fmt::Debug,
{
    fn from_iter<I: IntoIterator<Item = Either<X, T>>>(iter: I) -> Self {
        ListForm(ListFormGeneric::from_iter(iter))
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
