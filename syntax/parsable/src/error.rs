mod parse;

use std::ops::Range;

pub use self::parse::{ChumskyError, ParseError};

pub type Span = Range<usize>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io error: {0}")]
    Load(#[from] anyhow::Error),
    #[error("parse error: {0}")]
    Parse(#[from] ParseError),
}
