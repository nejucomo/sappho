use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

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
        self.to_string().load()
    }
}

impl LoadSource for String {
    fn load(self) -> Result<SourceCodeLink> {
        Ok(SourceCode::new(Source::Memory, self).into())
    }
}

impl LoadSource for &Path {
    fn load(self) -> Result<SourceCodeLink> {
        self.to_path_buf().load()
    }
}

impl LoadSource for PathBuf {
    fn load(self) -> Result<SourceCodeLink> {
        let f = std::fs::File::open(&self)
            .context(format!("while opening path {:?}", self.display()))?;

        (Source::from(self), f).load()
    }
}

impl LoadSource for std::io::Stdin {
    fn load(self) -> Result<SourceCodeLink> {
        (Source::Stdin, self).load()
    }
}

impl<R> LoadSource for (Source, R)
where
    R: std::io::Read,
{
    fn load(self) -> Result<SourceCodeLink> {
        let (source, r) = self;
        let code =
            std::io::read_to_string(r).context(format!("while attempting to read {}", &source))?;
        Ok(SourceCode::new(source, code).into())
    }
}
