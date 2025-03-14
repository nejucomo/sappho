//! Provides the [WithSource] type for tracking source with parse outputs
//!
//! This is especially useful for error displays.
#![deny(missing_docs)]

use chumsky::Parser as _;
use derive_new::new;
use sappho_parsable::{ParsableWith, Parser};
use sappho_source::{SourceCodeLink, SourceCodeRef};
use sappho_unparse::Unparse;

/// Associate parsed data with the code from which it came
#[derive(Debug, new)]
pub struct WithSource<T> {
    parsed: T,
    sc: SourceCodeRef,
}

impl<T> WithSource<T> {
    /// Refer to data parsed from this code
    pub fn parsed(&self) -> &T {
        &self.parsed
    }

    /// Refer to the source of this code
    pub fn sourcecode(&self) -> &SourceCodeRef {
        &self.sc
    }
}

impl<'l, T, P> ParsableWith<(&'l SourceCodeLink, T)> for WithSource<P>
where
    P: ParsableWith<T>,
{
    fn make_parser_with((sc, t): (&'l SourceCodeLink, T)) -> impl Parser<Self> {
        P::parser_with(t).map_with_span(|p, span| WithSource::new(p, sc.refer_to_span(span)))
    }
}

impl<P> Unparse for WithSource<P>
where
    P: Unparse,
{
    fn unparse_into(&self, s: &mut sappho_unparse::Stream) {
        self.parsed.unparse_into(s)
    }
}
