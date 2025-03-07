use chumsky::Parser as _;
use derive_more::{Constructor, From};
use either::Either;
use sappho_syntax_parsable::{Parser, Recursive, RecursiveParsable};
use sappho_syntax_unparse::{Stream, Unparse};
use sappho_try_transform::TryTransformInto;

#[derive(Clone, Debug, Eq, PartialEq, From, Constructor)]
pub struct LeftAssoc<L, R> {
    pub left: L,
    pub rights: Vec<R>,
}

impl<X, L, R> RecursiveParsable<X> for LeftAssoc<L, R>
where
    L: RecursiveParsable<X>,
    R: RecursiveParsable<X>,
{
    fn recursive_parser(rec: Recursive<'_, X>) -> impl Parser<Self> {
        L::recursive_parser(rec.clone())
            .then(R::recursive_parser(rec).repeated())
            .map(LeftAssoc::from)
    }
}

impl<L, R> Unparse for LeftAssoc<L, R>
where
    L: Unparse,
    R: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        s.write(&self.left);
        for r in &self.rights {
            s.write(r);
        }
    }
}

// Test Conversions:
impl<L, R> From<L> for LeftAssoc<L, R> {
    fn from(left: L) -> Self {
        Self::from((left, vec![]))
    }
}

impl<L, R> TryTransformInto<L> for LeftAssoc<L, R> {
    fn try_transform_into(self) -> Either<L, Self> {
        use either::Either::{Left, Right};

        if self.rights.is_empty() {
            Left(self.left)
        } else {
            Right(self)
        }
    }
}
