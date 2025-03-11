use chumsky::Parser as _;
use sappho_parsable::{ParsableWith, Parser};
use sappho_unparse::Unparse;

#[derive(Debug, derive_more::From)]
pub struct LeftAssoc<L, R> {
    left: L,
    rights: Vec<R>,
}

impl<L, R, T> ParsableWith<T> for LeftAssoc<L, R>
where
    L: ParsableWith<T>,
    R: ParsableWith<T>,
    T: Clone,
{
    fn make_parser_with(t: T) -> impl Parser<Self> {
        L::parser_with(t.clone())
            .then(R::parser_with(t).repeated())
            .map(LeftAssoc::from)
    }
}

impl<L, R> Unparse for LeftAssoc<L, R>
where
    L: Unparse,
    R: Unparse,
{
    fn unparse_into(&self, s: &mut sappho_unparse::Stream) {
        s.write(&self.left);
        for r in &self.rights {
            s.write(r);
        }
    }
}
