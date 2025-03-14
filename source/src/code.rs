use derive_more::{From, Into};
use derive_new::new;

use crate::Source;

/// [SourceCode] refers to the textual source code and tracks the [Source] it came from (if any).
#[derive(Clone, Debug, new, From, Into)]
pub struct SourceCode {
    #[new(into)]
    source: Source,
    #[new(into)]
    code: String,
}

impl SourceCode {
    /// Reference the source
    pub fn source(&self) -> &Source {
        &self.source
    }

    /// Reference the code
    pub fn code(&self) -> &str {
        self.code.as_ref()
    }
}
