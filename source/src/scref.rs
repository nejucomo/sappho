use std::ops::Range;

use derive_more::Deref;
use derive_new::new;

use crate::SourceCodeLink;

/// A span is a byte range within [SourceCode](crate::SourceCode) from which data was parsed
pub type Span = Range<usize>;

/// Refers to a specific span of code within a [Source](crate::Source)
#[derive(Clone, Debug, PartialEq, Eq, new, Deref)]
pub struct SourceCodeRef {
    #[deref]
    link: SourceCodeLink,
    span: Span,
}

impl SourceCodeRef {
    /// The [SourceCodeLink] to which this refers
    pub fn link(&self) -> &SourceCodeLink {
        &self.link
    }

    /// Refer to the specific code substring with the given span
    pub fn code(&self) -> &str {
        &self.link().code()[self.span()]
    }

    /// Refer to our specific span within the full source (which available as [Self::link().code()])
    pub fn span(&self) -> Span {
        self.span.clone()
    }
}
