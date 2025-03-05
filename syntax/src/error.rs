use std::ops::Range;
use std::path::PathBuf;

pub type Error = chumsky::error::Simple<char, Span>;
pub type Span = (Option<PathBuf>, Range<usize>);
