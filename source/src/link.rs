use std::rc::Rc;

use derive_more::{Deref, From};

use crate::scref::Span;
use crate::{SourceCode, SourceCodeRef};

/// A reference counted link to [SourceCode]
///
/// Often multiple parsed items need to refer back to the source, and this meets that need.
#[derive(Clone, Debug, From, Deref)]
#[from(SourceCode)]
pub struct SourceCodeLink(Rc<SourceCode>);

impl SourceCodeLink {
    /// Refer to a specific span
    pub fn refer_to_span(&self, span: Span) -> SourceCodeRef {
        SourceCodeRef::new(self.clone(), span)
    }
}
