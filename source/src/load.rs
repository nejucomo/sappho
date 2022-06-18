use crate::Source;
use std::io::Result;
use std::path::Path;

/// Load a [Source] via [std::io::Result]. Impls are provided for [String], [str], and [Path].
pub trait LoadSource<'a> {
    fn load(self) -> Result<Source<'a>>;
}

impl<'a> LoadSource<'a> for &'a str {
    fn load(self) -> Result<Source<'a>> {
        Ok(Source::wrap_string(self))
    }
}

impl<'a> LoadSource<'a> for String {
    fn load(self) -> Result<Source<'a>> {
        Ok(Source::wrap_string(self))
    }
}

impl<'a> LoadSource<'a> for &'a Path {
    fn load(self) -> Result<Source<'a>> {
        Source::load_path(self)
    }
}

impl<'a> LoadSource<'a> for (&'a Path, &'a str) {
    fn load(self) -> Result<Source<'a>> {
        let (path, text) = self;
        Ok(Source::wrap(path, text))
    }
}
