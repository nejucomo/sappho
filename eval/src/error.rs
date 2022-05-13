use derive_more::From;

#[derive(Debug, From)]
pub enum Error {
    Parse(Vec<saplang_parser::Error>),
}

pub type Result<T> = std::result::Result<T, Error>;
