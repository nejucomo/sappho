use anyhow::Result;
use std::borrow::Cow;
use std::path::Path;

// Todo: Replace with `source-text` crate.

/// A `Source` refers to the textual source code and tracks the [Path] it came from (if any).
#[derive(Clone, Debug)]
pub struct Source<'a> {
    optpath: Option<&'a Path>,
    cowtext: Cow<'a, str>,
}

impl<'a> Source<'a> {
    /// Load source from a [Path].
    pub fn load_path(path: &Path) -> Result<Source> {
        use anyhow_std::PathAnyhow;

        let text = path.read_to_string_anyhow()?;
        Ok(Source {
            optpath: Some(path),
            cowtext: Cow::from(text),
        })
    }

    /// Wrap a source string.
    pub fn wrap_string<S>(text: S) -> Source<'a>
    where
        Cow<'a, str>: From<S>,
    {
        Source {
            optpath: None,
            cowtext: Cow::from(text),
        }
    }

    /// The [Path] this source was loaded from, if any.
    pub fn path(&self) -> Option<&Path> {
        self.optpath
    }

    /// The source code text of this source.
    pub fn text(&self) -> &str {
        self.cowtext.as_ref()
    }

    pub(crate) fn wrap(path: &'a Path, text: &'a str) -> Self {
        Source {
            optpath: Some(path),
            cowtext: Cow::from(text),
        }
    }
}
