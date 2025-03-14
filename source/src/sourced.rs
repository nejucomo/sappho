use std::ops::Range;

use derive_more::From;

use crate::{Source, SourceCodeLink};

#[derive(Debug, From)]
pub struct Sourced<T> {
    data: T,
    sclink: SourceCodeLink,
    span: Span,
}

pub type Span = Range<usize>;

impl<T> Sourced<T> {
    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn source(&self) -> &Source {
        self.sclink.source()
    }

    pub fn span(&self) -> &Span {
        &self.span
    }

    pub fn all_code(&self) -> &str {
        self.sclink.code()
    }
}
