use crate::ValRef;
use derive_more::From;
use saplang_ast::Identifier;

#[derive(Debug, From)]
pub enum Error {
    Parse(Vec<saplang_parser::Error>),
    Unbound(Identifier),
    Uncallable(ValRef),
}

pub type Result<T> = std::result::Result<T, Error>;
