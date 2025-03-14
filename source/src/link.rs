use std::rc::Rc;

use derive_more::From;

use crate::{Source, SourceCode};

#[derive(Debug, From)]
pub struct SourceCodeLink(Rc<SourceCode<String>>);

impl SourceCodeLink {
    pub fn source(&self) -> &Source {
        self.0.source()
    }

    pub fn code(&self) -> &str {
        self.0.code()
    }
}
