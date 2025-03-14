use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::{Source, SourceCode, SourceCodeLink};

/// Types which can load a [SourceCodeLink]
pub trait LoadSource: Sized {
    /// Load a [SourceCodeLink]
    fn load(self) -> Result<SourceCodeLink>;
}

impl LoadSource for SourceCodeLink {
    fn load(self) -> Result<SourceCodeLink> {
        Ok(self)
    }
}

impl LoadSource for SourceCode {
    fn load(self) -> Result<SourceCodeLink> {
        Ok(SourceCodeLink::from(self))
    }
}

impl LoadSource for &str {
    fn load(self) -> Result<SourceCodeLink> {
        Ok(SourceCode::new(Source::default(), self).into())
    }
}

impl LoadSource for String {
    fn load(self) -> Result<SourceCodeLink> {
        Ok(SourceCode::new(Source::default(), self).into())
    }
}

impl LoadSource for &Path {
    fn load(self) -> Result<SourceCodeLink> {
        self.to_path_buf().load()
    }
}

impl LoadSource for PathBuf {
    fn load(self) -> Result<SourceCodeLink> {
        use anyhow_std::PathAnyhow;

        let code = self.read_to_string_anyhow()?;
        let source = Source::from(self);
        Ok(SourceCode::new(source, code).into())
    }
}
