use crate::CodeOrigin;
use anyhow::Result;
use std::path::Path;

/// Load a [Source] via [std::io::Result]. Impls are provided for [String], [str], and [Path].
pub trait LoadSource<'a> {
    fn load(self) -> Result<CodeOrigin<'a>>;
}

impl<'a> LoadSource<'a> for &'a str {
    fn load(self) -> Result<CodeOrigin<'a>> {
        Ok(CodeOrigin::wrap_string(self))
    }
}

impl<'a> LoadSource<'a> for String {
    fn load(self) -> Result<CodeOrigin<'a>> {
        Ok(CodeOrigin::wrap_string(self))
    }
}

impl<'a> LoadSource<'a> for &'a Path {
    fn load(self) -> Result<CodeOrigin<'a>> {
        CodeOrigin::load_path(self)
    }
}

impl<'a> LoadSource<'a> for (&'a Path, &'a str) {
    fn load(self) -> Result<CodeOrigin<'a>> {
        let (path, text) = self;
        Ok(CodeOrigin::wrap(path, text))
    }
}
