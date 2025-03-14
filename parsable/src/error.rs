mod parse;
mod serror;

pub use self::parse::ParseError;
pub use self::serror::ChumskyError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("source loading error: {0}")]
    Load(#[from] anyhow::Error),
    #[error("parse error: {0}")]
    Parse(#[from] ParseError),
}
