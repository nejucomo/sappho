mod lfg;
mod lfiter;
mod parsable;

use std::fmt;

use either::Either;
use sappho_unparse::Unparse;

use self::lfg::ListFormGeneric;
pub use self::lfiter::ListFormIter;

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

    pub(crate) fn lfg_ref(&self) -> ListFormGeneric<&Vec<X>, &T> {
        self.0.as_ref()
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

#[cfg(test)]
mod tests;
