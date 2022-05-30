use crate::ValRef;
use derive_more::From;
use sappho_ast::Identifier;

#[derive(Debug, From)]
pub enum Error {
    Unbound(Identifier),
    Uncallable(ValRef),
}

pub type Result<T> = std::result::Result<T, Error>;
