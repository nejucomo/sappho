use sappho_unparse::Unparse;

use crate::Parser;

pub trait Parsable: Sized + Unparse {
    fn parser() -> impl Parser<Self>;
}
