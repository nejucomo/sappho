//! Provides the [WithSource] type for tracking source with parse outputs
//!
//! This is especially useful for error displays.
#![deny(missing_docs)]

use chumsky::Parser as _;
use derive_more::Into;
use derive_new::new;
use sappho_parsable::{ParsableWith, Parser};
use sappho_source::{SourceCodeLink, SourceCodeRef};
use sappho_unparse::Unparse;

/// Associate parsed data with the code from which it came
#[derive(Clone, Debug, PartialEq, Eq, new, Into)]
pub struct WithSource<T> {
    /// The parsed item
    pub parsed: T,
    /// The [SourceCodeRef] from which [Self::parsed] came
    pub sourcecode: SourceCodeRef,
}

impl<T> WithSource<T> {
    /// Get a reference to [Self::parsed] which carries the [SourceCodeRef]
    pub fn as_ref(&self) -> WithSource<&T> {
        WithSource {
            parsed: &self.parsed,
            sourcecode: self.sourcecode.clone(),
        }
    }

    /// Map the parsed value while retaining the [SourceCodeRef]
    pub fn map<F, U>(self, f: F) -> WithSource<U>
    where
        F: FnOnce(T) -> U,
    {
        WithSource {
            parsed: f(self.parsed),
            sourcecode: self.sourcecode,
        }
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
