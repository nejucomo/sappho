use derive_more::From;

#[derive(Debug, From)]
pub enum Error {
    Std(std::io::Error),
    Parse(sappho_parser::Errors),
}

pub type Result<T> = std::result::Result<T, Error>;
