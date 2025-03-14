use std::fmt::Debug;

use sappho_source::Source;
use sappho_unparse::Unparse;

use crate::error::Error;
use crate::Parser;

pub trait Parsable: Sized + Unparse + Debug {
    fn load_and_parse<S>(source: S) -> Result<Self, Error>
    where
        Source: From<S>,
    {
        Self::parser().load_and_parse(source)
    }

    fn parser() -> impl Parser<Self>;
}
