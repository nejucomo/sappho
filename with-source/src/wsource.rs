use derive_more::Into;
use derive_new::new;
use sappho_source::SourceCodeRef;

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
