mod parse;

use std::ops::Range;

pub use self::parse::{ChumskyError, ParseError};

pub type Span = Range<usize>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Load(#[from] anyhow::Error),
    #[error(transparent)]
    Parse(#[from] ParseError),
}
