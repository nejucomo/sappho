use chumsky::Parser as _;
use sappho_effect::{RestrictFrom, Restriction};
use sappho_parsable::{ParsableWith, Parser};
use sappho_source::SourceCodeLink;
use sappho_unparse::Unparse;

use crate::wsource::WithSource;

impl<'l, T, P> ParsableWith<(&'l SourceCodeLink, T)> for WithSource<P>
where
    P: ParsableWith<T>,
{
    fn make_parser_with((sc, t): (&'l SourceCodeLink, T)) -> impl Parser<Self> {
        P::parser_with(t).map_with_span(|p, span| WithSource::new(p, sc.refer_to_span(span)))
    }
}

impl<P> Unparse for WithSource<P>
where
    P: Unparse,
{
    fn unparse_into(&self, s: &mut sappho_unparse::Stream) {
        self.parsed.unparse_into(s)
    }
}

impl<S, T> RestrictFrom<WithSource<S>> for WithSource<T>
where
    T: RestrictFrom<S>,
{
    fn restrict(src: WithSource<S>) -> Result<Self, Restriction> {
        src.map(T::restrict).transpose()
    }
}
