use std::borrow::Cow;
use std::io::Result;
use std::path::Path;

/// A `Source` refers to the textual source code and tracks the [Path] it came from (if any).
pub struct Source<'a> {
    optpath: Option<&'a Path>,
    cowtext: Cow<'a, str>,
}

impl<'a> Source<'a> {
    /// Load source from a [Path].
    pub fn load_path(path: &Path) -> Result<Source> {
        use pathutil::PathExt;

        let text = path.pe_read_to_string()?;
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
}
