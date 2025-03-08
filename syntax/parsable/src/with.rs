use chumsky::Parser as _;
use sappho_syntax_unparse::Unparse;

use crate::{Parsable, Parser};

pub trait ParsableWith<T>: Sized + Unparse + std::fmt::Debug {
    /// Consumers should call this to support parser debugging
    fn parser_with(param: T) -> impl Parser<Self> {
        Self::make_parser_with(param).map(|s| dbg!(s))
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
