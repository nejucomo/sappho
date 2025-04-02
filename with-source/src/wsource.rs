use derive_more::Into;
use derive_new::new;
use sappho_source::SourceCodeRef;

/// Associate parsed data with the code from which it came
#[derive(Clone, Debug, PartialEq, Eq, new, Into)]
pub struct WithSource<T> {
    /// The parsed item
    #[new(into)]
    parsed: T,
    /// The [SourceCodeRef] from which [Self::parsed] came
    #[new(into)]
    sourcecode: Option<SourceCodeRef>,
}

impl<T> WithSource<T> {
    /// Refer to the parsed value
    pub fn parsed(&self) -> &T {
        &self.parsed
    }

    /// Get the [SourceCodeRef] if any
    pub fn sourcecode(&self) -> Option<&SourceCodeRef> {
        self.sourcecode.as_ref()
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

    /// Just return the parsed value; ignoring the source
    pub fn ignore_source(self) -> T {
        self.parsed
    }
}

impl<T, E> WithSource<Result<T, E>> {
    /// Transpose an `Result<T, E>` parsed value
    pub fn transpose(self) -> Result<WithSource<T>, E> {
        let WithSource {
            parsed: res,
            sourcecode,
        } = self;

        res.map(|parsed| WithSource { parsed, sourcecode })
    }
}

impl<T> From<T> for WithSource<T> {
    fn from(parsed: T) -> Self {
        WithSource {
            parsed,
            sourcecode: None,
        }
    }
}
