use sappho_syntax_unparse::Unparse;

use crate::error::ChumskyError;
use crate::Parser;

pub type Recursive<'a, O> = chumsky::recursive::Recursive<'a, char, O, ChumskyError>;

pub trait RecursiveParsable<T>: Sized + Unparse {
    fn recursive_parser(rec: Recursive<'_, T>) -> impl Parser<Self>;
}
