use either::Either::{self, Left, Right};
use sappho_unparse::{Stream, Unparse};
use std::fmt;

use crate::SeqAndTail;

/// A general structure for a sequence of items, with an optional tail, used for both list patterns
/// and expressions in the ast, examples: `[]`, `[32]`, `[a, b, ..t]`
#[derive(Clone, Debug, PartialEq)]
pub struct ListForm<Elem, Tail>(SeqAndTail<Vec<Elem>, Tail>);

impl<X, T> ListForm<X, T> {
    pub fn new<I>(body: I, tail: Option<T>) -> Self
    where
        I: IntoIterator<Item = X>,
    {
        body.into_iter().map(Left).chain(tail.map(Right)).collect()
    }

    pub fn is_empty(&self) -> bool {
        self.0
            .as_ref()
            .extract(|body, tail| body.is_empty() && tail.is_none())
    }

    pub fn map_elems<F, DX>(self, f: F) -> ListForm<DX, T>
    where
        F: Fn(X) -> DX,
    {
        self.into_iter().map(|ei| ei.map_left(&f)).collect()
    }

    pub fn map_tail<F, DT>(self, f: F) -> ListForm<X, DT>
    where
        F: Fn(T) -> DT,
    {
        ListForm(self.0.map_tail(f))
    }

    pub fn into_reverse_fold<S, TT, F>(self, ttail: TT, f: F) -> S
    where
        TT: FnOnce(Option<T>) -> S,
        F: Fn(S, X) -> S,
    {
        self.0.into_reverse_fold(ttail, f)
    }

    pub fn try_map<TX, DX, TT, DT, E>(
        self,
        try_map_elem: TX,
        try_map_tail: TT,
    ) -> Result<ListForm<DX, DT>, E>
    where
        TX: Fn(X) -> Result<DX, E>,
        TT: FnOnce(T) -> Result<DT, E>,
    {
        self.0.extract(|body, optail| {
            let body = body
                .into_iter()
                .map(try_map_elem)
                .collect::<Result<_, _>>()?;
            let optail = optail.map(try_map_tail).transpose()?;
            Ok(ListForm(SeqAndTail::new(body, optail)))
        })
    }
}

impl<X, T, E> ListForm<X, Result<T, E>> {
    pub fn transpose_tail(self) -> Result<ListForm<X, T>, E> {
        self.0.extract(|body, optail| {
            let optail = optail.transpose()?;
            Ok(ListForm(SeqAndTail::new(body, optail)))
        })
    }
}

impl<X, T> IntoIterator for ListForm<X, T> {
    type Item = Either<X, T>;
    type IntoIter = SeqAndTail<<Vec<X> as IntoIterator>::IntoIter, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iterator()
    }
}

impl<X, T> FromIterator<Either<X, T>> for ListForm<X, T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Either<X, T>>,
    {
        ListForm(iter.into_iter().collect())
    }
}

impl<X, T> Unparse for ListForm<X, T>
where
    X: Unparse,
    T: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        use sappho_unparse::Brackets::Square;
        use sappho_unparse::Break::OptSpace;

        if self.is_empty() {
            s.write("[]")
        } else {
            s.bracketed(Square, |subs| {
                let mut first = true;

                self.0.as_ref().extract(|body, optail| {
                    for elem in body.iter() {
                        if first {
                            first = false;
                        } else {
                            subs.write(",");
                        }
                        subs.write(&OptSpace);
                        subs.write(elem);
                    }

                    if let Some(tail) = optail {
                        if !first {
                            subs.write(",");
                        }
                        subs.write(&OptSpace);
                        subs.write("..");
                        subs.write(tail);
                    }
                });
            });
        }
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

#[cfg(test)]
mod tests {
    use crate::ListForm;
    use indoc::indoc;
    use sappho_unparse::{Stream, Unparse};
    use test_case::test_case;

    struct X;

    impl Unparse for X {
        fn unparse_into(&self, s: &mut Stream) {
            s.write("X");
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
