//! A crate for loading source code while tracking its provenance.

use std::borrow::Cow;
use std::path::Path;

use derive_new::new;

/// Code along with a description of its origin
#[derive(Clone, Debug, new)]
pub struct CodeOrigin<'a> {
    #[new(into)]
    code: Cow<'a, str>,
    #[new(into)]
    origin: Cow<'a, str>,
}

impl<'a> CodeOrigin<'a> {
    /// Load source from a [Path] or [PathBuf](std::path::PathBuf).
    pub fn load_path(path: impl AsRef<Path>) -> anyhow::Result<CodeOrigin<'a>> {
        use anyhow_std::PathAnyhow as _;

        let pref = path.as_ref();
        let code = pref.read_to_string_anyhow()?;

        let origin = format!("file {:?}", pref.display());
        Ok(CodeOrigin::new(origin, code))
    }

    /// The origin of this code
    pub fn origin(&self) -> &str {
        self.origin.as_ref()
    }

    /// The source code from this origin
    pub fn code(&self) -> &str {
        self.code.as_ref()
    }
}
