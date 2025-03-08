use std::fmt::Debug;

use chumsky::Parser as _;
use sappho_syntax_unparse::Unparse;

use crate::{Parsable, Parser};

pub trait ParsableWith<T>: Sized + Unparse + Debug {
    /// Consumers should call this to support parser debugging
    fn parser_with(param: T) -> impl Parser<Self> {
        Self::make_parser_with(param).map(move |parsed| dbg!(parsed))
    }

    /// Implementors should implement this to support parser debugging
    fn make_parser_with(param: T) -> impl Parser<Self>;
}

impl<P> Parsable for P
where
    P: ParsableWith<()>,
{
    fn parser() -> impl Parser<Self> {
        P::parser_with(())
    }
}

impl<T, P> ParsableWith<T> for Box<P>
where
    P: ParsableWith<T>,
{
    fn make_parser_with(param: T) -> impl Parser<Self> {
        P::parser_with(param).map(Box::new)
    }
}
