use std::fmt::Debug;

use sappho_source::LoadSource;
use sappho_unparse::Unparse;

use crate::error::Error;
use crate::Parser;

pub trait Parsable: Sized + Unparse + Debug {
    fn load_and_parse<L>(loadable: L) -> Result<Self, Error>
    where
        L: LoadSource,
    {
        Self::parser().load_and_parse(loadable)
    }

    fn parser() -> impl Parser<Self>;
}
