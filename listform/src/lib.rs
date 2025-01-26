use std::fmt;

use either::Either::{self, Left, Right};
use sappho_unparse::{self as unparse, Unparse, UnparseContainer};

/// A general structure for a sequence of items, with an optional tail, used for both list patterns
/// and expressions in the ast, examples: `[]`, `[32]`, `[a, b, ..t]`
#[derive(Clone, Debug, PartialEq)]
pub struct ListForm<Elem, Tail> {
    body: Vec<Elem>,
    tail: Option<Tail>,
}

impl<X, T> ListForm<X, T> {
    pub fn new<I>(body: I, tail: Option<T>) -> Self
    where
        I: IntoIterator<Item = X>,
    {
        ListForm {
            body: body.into_iter().collect(),
            tail,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.body.is_empty() && self.tail.is_none()
    }

    pub fn map_elems<F, DX>(self, f: F) -> ListForm<DX, T>
    where
        F: Fn(X) -> DX,
    {
        ListForm {
            body: self.body.into_iter().map(f).collect(),
            tail: self.tail,
        }
    }

    pub fn map_tail<F, DT>(self, f: F) -> ListForm<X, DT>
    where
        F: Fn(T) -> DT,
    {
        ListForm {
            body: self.body,
            tail: self.tail.map(f),
        }
    }

    pub fn into_reverse_fold<S, TT, F>(self, ttail: TT, f: F) -> S
    where
        TT: FnOnce(Option<T>) -> S,
        F: Fn(S, X) -> S,
    {
        self.body.into_iter().rev().fold(ttail(self.tail), f)
    }

    pub fn try_map<TX, DX, TT, DT, E>(self, telem: TX, ttail: TT) -> Result<ListForm<DX, DT>, E>
    where
        TX: Fn(X) -> Result<DX, E>,
        TT: FnOnce(T) -> Result<DT, E>,
    {
        let bodyres: Result<Vec<DX>, E> = self.body.into_iter().map(telem).collect();

        Ok(ListForm {
            body: bodyres?,
            tail: self.tail.map(ttail).transpose()?,
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = Either<&X, &T>> {
        self.body
            .iter()
            .map(Left)
            .chain(self.tail.iter().map(Right))
    }
}

impl<X, T, E> ListForm<X, Result<T, E>> {
    pub fn transpose_tail(self) -> Result<ListForm<X, T>, E> {
        Ok(ListForm {
            body: self.body,
            tail: self.tail.transpose()?,
        })
    }
}

impl<X, T> UnparseContainer for ListForm<X, T>
where
    X: Unparse,
    T: Unparse,
{
    fn unparse_header<'a, 'b>(&self, stream: &mut unparse::Stream<'a, 'b>) -> unparse::Result<()> {
        stream.write("[")
    }

    fn unparse_footer<'a, 'b>(&self, stream: &mut unparse::Stream<'a, 'b>) -> unparse::Result<()> {
        stream.write("]")
    }

    type UnparseChild<'s>
        = Either<&'s X, &'s T>
    where
        X: 's,
        T: 's;

    fn unparse_iter<'s>(&'s self) -> impl Iterator<Item = Self::UnparseChild<'s>> {
        self.iter()
    }

    fn unparse_separator() -> &'static str {
        ","
    }
}

impl<X, T> Unparse for ListForm<X, T>
where
    X: Unparse,
    T: Unparse,
{
    fn unparse<'a, 'b>(&self, stream: &mut unparse::Stream<'a, 'b>) -> unparse::Result<()> {
        self.unparse_container(stream)
    }
}

impl<X, T> fmt::Display for ListForm<X, T>
where
    X: Unparse,
    T: Unparse,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unparse::to_formatter(self, f, 80)
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use sappho_unparse::{self as unparse, Unparse};
    use test_case::test_case;

    use crate::ListForm;

    struct X;

    impl Unparse for X {
        fn unparse<'a, 'b>(&self, stream: &mut unparse::Stream<'a, 'b>) -> unparse::Result<()> {
            stream.write("X")
        }
    }

    #[test_case([], None => "[]")]
    #[test_case([], Some(X) => indoc! { "
        [
          ..X
        ]"
    })]
    #[test_case([X], None => indoc! { "
        [
          X
        ]"
    })]
    #[test_case([X], Some(X) => indoc! { "
        [
          X,
          ..X
        ]"
    })]
    #[test_case([X, X], Some(X) => indoc! { "
        [
          X,
          X,
          ..X
        ]"
    })]
    fn display<const K: usize>(body: [X; K], tail: Option<X>) -> String {
        ListForm::new(body, tail).to_string()
    }
}
