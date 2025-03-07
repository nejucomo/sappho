use sappho_syntax_unparse::Unparse;

use crate::error::ChumskyError;
use crate::{Parsable, Parser};

pub type Recursive<'a, O> = chumsky::recursive::Recursive<'a, char, O, ChumskyError>;

pub trait RecursiveParsable<T>: Sized + Unparse {
    fn recursive_parser(rec: Recursive<'_, T>) -> impl Parser<Self>;
}

impl<T, P> RecursiveParsable<T> for P
where
    P: Parsable,
{
    fn recursive_parser(_: Recursive<'_, T>) -> impl Parser<Self> {
        P::parser()
    }
}
