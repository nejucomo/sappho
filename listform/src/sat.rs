use either::Either::{self, Left, Right};
use sappho_legible::{BracketSeq, IntoNode, Node};

#[derive(Clone, Debug, PartialEq, derive_new::new)]
pub struct SeqAndTail<S, T> {
    seq: S,
    tail: Option<T>,
}

impl<S, T> SeqAndTail<S, T> {
    pub fn extract<F, R>(self, f: F) -> R
    where
        F: FnOnce(S, Option<T>) -> R,
    {
        f(self.seq, self.tail)
    }

    pub fn map_sequence<F, MS>(self, f: F) -> SeqAndTail<MS, T>
    where
        F: FnOnce(S) -> MS,
    {
        SeqAndTail::new(f(self.seq), self.tail)
    }

    pub fn map_opt_tail<F, MT>(self, f: F) -> SeqAndTail<S, MT>
    where
        F: FnOnce(Option<T>) -> Option<MT>,
    {
        SeqAndTail::new(self.seq, f(self.tail))
    }

    pub fn map_tail<F, MT>(self, f: F) -> SeqAndTail<S, MT>
    where
        F: FnOnce(T) -> MT,
    {
        self.map_opt_tail(|ot| ot.map(f))
    }

    pub fn into_reverse_fold<A, TT, F>(self, ttail: TT, f: F) -> A
    where
        S: IntoIterator,
        S::IntoIter: DoubleEndedIterator,
        TT: FnOnce(Option<T>) -> A,
        F: Fn(A, S::Item) -> A,
    {
        self.seq.into_iter().rev().fold(ttail(self.tail), f)
    }

    pub fn as_ref(&self) -> SeqAndTail<&S, &T> {
        SeqAndTail::new(&self.seq, self.tail.as_ref())
    }

    pub fn into_iterator(self) -> SeqAndTail<S::IntoIter, T>
    where
        S: IntoIterator,
    {
        self.map_sequence(S::into_iter)
    }
}

impl<S, X, T> IntoNode for SeqAndTail<S, T>
where
    S: IntoIterator<Item = X>,
    X: IntoNode,
    T: IntoNode,
{
    fn into_node(self) -> Node {
        BracketSeq::new(
            ("[%", "%]"),
            ", ",
            self.into_iterator()
                .map(|ei| ei.either(|x| x.into_node(), |t| ("..", t).into_node())),
        )
        .into_node()
    }
}

impl<I, T> Iterator for SeqAndTail<I, T>
where
    I: Iterator,
{
    type Item = Either<I::Item, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.seq
            .next()
            .map(Left)
            .or_else(|| self.tail.take().map(Right))
    }
}

impl<S, X, T> FromIterator<Either<X, T>> for SeqAndTail<S, T>
where
    S: Default + Extend<X>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Either<X, T>>,
    {
        let mut body = S::default();
        let mut tail = None;

        let mut it = iter.into_iter();
        for ei in it.by_ref() {
            match ei {
                Left(x) => body.extend(Some(x)),
                Right(t) => {
                    tail.replace(t);
                    break;
                }
            }
        }

        assert!(it.next().is_none(), "items after `Right` tail");

        SeqAndTail::new(body, tail)
    }
}
