use crate::Source;

// Todo: Replace with `source-text` crate.

/// [SourceCode] refers to the textual source code and tracks the [Source] it came from (if any).
#[derive(Clone, Debug)]
pub struct SourceCode<C>
where
    C: AsRef<str>,
{
    source: Source,
    code: C,
}

impl<C> SourceCode<C>
where
    C: AsRef<str>,
{
    pub(crate) fn new(source: Source, code: C) -> Self {
        SourceCode { source, code }
    }

    pub fn source(&self) -> &Source {
        &self.source
    }

    pub fn code(&self) -> &str {
        self.code.as_ref()
    }

    pub fn to_owned(self) -> SourceCode<String>
    where
        String: From<C>,
    {
        let SourceCode { source, code } = self;
        let code = String::from(code);
        SourceCode { source, code }
    }
}
