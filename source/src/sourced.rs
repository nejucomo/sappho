use std::ops::Range;

use derive_more::From;

use crate::{Source, SourceCodeLink};

/// Associate parsed data with the code from which it came
#[derive(Debug, From)]
pub struct Sourced<T> {
    parsed: T,
    sclink: SourceCodeLink,
    span: Span,
}

/// A span is a range within [Sourced::all_code] from which data was parsed
pub type Span = Range<usize>;

impl<T> Sourced<T> {
    /// Refer to data parsed from this code
    pub fn parsed(&self) -> &T {
        &self.parsed
    }

    /// Refer to the source of this code
    pub fn source(&self) -> &Source {
        self.sclink.source()
    }

    /// Refer to the span within [Self::all_code] from which [Self::parsed] comes
    pub fn span(&self) -> Span {
        self.span.clone()
    }

    /// Refer to the specific code substring from which [Self::parsed] comes
    pub fn code(&self) -> &str {
        &self.all_code()[self.span()]
    }

    /// Refer to the entire code
    pub fn all_code(&self) -> &str {
        self.sclink.code()
    }
}
