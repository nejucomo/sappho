use sappho_syntax_unparse::Unparse;

use crate::Parser;

pub trait Parsable: Sized + Unparse {
    fn parser() -> impl Parser<Self>;
}
