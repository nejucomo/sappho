use std::fmt::Debug;

use chumsky::Parser as _;
use sappho_source::{Source, SourceCodeLink};
use sappho_unparse::Unparse;

use crate::error::{Error, ParseError};
use crate::Parser;

pub trait ParsableWith<T>: Sized + Unparse + Debug {
    fn load_and_parse<S>(source: S) -> Result<Self, Error>
    where
        T: for<'a> From<&'a SourceCodeLink>,
        Source: From<S>,
    {
        let sc = Source::from(source).load().map_err(Error::Load)?;
        let parsed = Self::parse_with_sourcecode(&sc)?;
        Ok(parsed)
    }

    fn parse_with_sourcecode<'a>(sc: &'a SourceCodeLink) -> Result<Self, ParseError>
    where
        T: From<&'a SourceCodeLink>,
    {
        Self::parser_with(T::from(sc)).parse_sourcecode(sc)
    }

    /// Consumers should call this to support parser debugging
    fn parser_with(param: T) -> impl Parser<Self> {
        Self::make_parser_with(param).map(move |parsed| dbg!(parsed))
    }

    /// Implementors should implement this to support parser debugging
    fn make_parser_with(param: T) -> impl Parser<Self>;
}

impl<T, P> ParsableWith<T> for Box<P>
where
    P: ParsableWith<T>,
{
    fn make_parser_with(param: T) -> impl Parser<Self> {
        P::parser_with(param).map(Box::new)
    }
}
