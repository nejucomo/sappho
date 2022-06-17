#[derive(Debug, derive_more::From)]
pub enum Error {
    Stdio(std::io::Error),
    Parse(sappho_parser::Errors),
    Eval(sappho_eval::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
