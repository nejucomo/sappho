use derive_new::new;

/// [SourceCode] refers to the textual source code and tracks the [Source](crate::Source) it came from (if any).
#[derive(Clone, Debug, new)]
pub struct SourceCode {
    #[new(into)]
    code: String,
    #[new(into)]
    source: String,
}

impl SourceCode {
    /// Reference the code
    pub fn code(&self) -> &str {
        self.code.as_str()
    }

    /// A user-centric description of the source
    pub fn source(&self) -> &str {
        self.source.as_str()
    }
}
