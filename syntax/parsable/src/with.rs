use sappho_syntax_unparse::Unparse;

use crate::{Parsable, Parser};

pub trait ParsableWith<T>: Sized + Unparse {
    fn parser_with(param: T) -> impl Parser<Self>;
}

impl<P> Parsable for P
where
    P: ParsableWith<()>,
{
    fn parser() -> impl Parser<Self> {
        P::parser_with(())
    }
}
