use either::Either::{self, Left, Right};
use sappho_unparse::{Stream, Unparse};
use std::fmt;

use crate::lfg::ListFormGeneric;
use crate::ListFormIter;

/// A general structure for a sequence of items, with an optional tail, used for both list patterns
/// and expressions in the ast, examples: `[]`, `[32]`, `[a, b, ..t]`
#[derive(Clone, Debug, PartialEq)]
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

                for xort in self.0.as_ref() {
                    match xort {
                        Left(elem) => {
                            if first {
                                first = false;
                            } else {
                                subs.write(",");
                            }
                            subs.write(&OptSpace);
                            subs.write(elem);
                        }
                        Right(tail) => {
                            if !first {
                                subs.write(",");
                            }
                            subs.write(&OptSpace);
                            subs.write("..");
                            subs.write(tail);
                        }
                    }
                }
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
