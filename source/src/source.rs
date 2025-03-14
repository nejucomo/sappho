use std::fmt;
use std::path::{Path, PathBuf};

/// The origin of a [SourceCode](crate::SourceCode)
///
/// This is an optional [PathBuf]. It is absent when source comes from arbitrary in-memory sources (especially in tests).
#[derive(Clone, Debug, Default)]
pub struct Source(Option<PathBuf>);

impl From<&Path> for Source {
    fn from(p: &Path) -> Self {
        Source::from(p.to_path_buf())
    }
}

impl From<PathBuf> for Source {
    fn from(p: PathBuf) -> Self {
        Source(Some(p))
    }
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(p) = self.0.as_ref() {
            write!(f, "{:?}", p.display())
        } else {
            write!(f, "<memory>")
        }
    }
}
