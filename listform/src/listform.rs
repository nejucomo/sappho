use either::Either::{self, Left, Right};
use sappho_legible::{Envelope, IntoNode, Legible, Node};
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

impl<'a, X, T> IntoNode for &'a ListForm<X, T>
where
    &'a X: IntoNode,
    &'a T: IntoNode,
{
    fn into_node(self) -> Node {
        Envelope::separated_bracketed_sequence(
            "[",
            ",",
            "]",
            self.0
                .as_ref()
                .into_iterator()
                .map(|ei| ei.either(|x| x.into_node(), |t| ("..", t).into_node())),
        )
        .into_node()
    }
}

impl<X, T> fmt::Display for ListForm<X, T>
where
    for<'a> &'a X: IntoNode,
    for<'a> &'a T: IntoNode,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_legible(f)
    }
}
