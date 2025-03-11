use chumsky::Parser as _;
use sappho_parsable::error::Span;
use sappho_parsable::{ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};

#[derive(Debug, derive_new::new)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

impl<T, A> ParsableWith<A> for Spanned<T>
where
    T: ParsableWith<A>,
{
    fn make_parser_with(param: A) -> impl Parser<Self> {
        T::parser_with(param).map_with_span(Self::new)
    }
}

impl<T> Unparse for Spanned<T>
where
    T: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        self.node.unparse_into(s)
    }
}
