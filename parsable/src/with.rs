use std::fmt::Debug;

use chumsky::Parser as _;

use crate::Parser;

pub trait ParsableWith<T>: Sized + Debug {
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
