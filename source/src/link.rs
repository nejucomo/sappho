use std::rc::Rc;

use derive_more::From;

use crate::{Source, SourceCode};

/// A reference counted link to [SourceCode]
///
/// Often multiple parsed items need to refer back to the source, and this meets that need.
#[derive(Debug, From)]
#[from(SourceCode)]
pub struct SourceCodeLink(Rc<SourceCode>);

impl SourceCodeLink {
    /// The source
    pub fn source(&self) -> &Source {
        self.0.source()
    }

    /// The code
    pub fn code(&self) -> &str {
        self.0.code()
    }
}
