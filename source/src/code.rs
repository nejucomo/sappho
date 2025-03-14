use derive_more::{From, Into};
use derive_new::new;

use crate::Source;

// Todo: Replace with `source-text` crate.
//
// TODO: refactor to be a ref-generic container.

/// [SourceCode] refers to the textual source code and tracks the [Source] it came from (if any).
#[derive(Clone, Debug, new, From, Into)]
pub struct SourceCode<C>
where
    C: AsRef<str>,
{
    #[new(into)]
    source: Source,
    code: C,
}

impl<C> SourceCode<C>
where
    C: AsRef<str>,
{
    pub fn source(&self) -> &Source {
        &self.source
    }

    pub fn code(&self) -> &str {
        self.code.as_ref()
    }

    pub fn to_owned(self) -> SourceCode<String>
    where
        C: ToString,
    {
        let SourceCode { source, code } = self;
        let code = code.to_string();
        SourceCode { source, code }
    }
}
