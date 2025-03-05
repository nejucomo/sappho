use std::path::Path;

use anyhow::Result;

use crate::{Source, SourceCode};

/// Load [SourceCode]
pub trait LoadSource<C>
where
    C: AsRef<str>,
{
    fn load(self) -> Result<SourceCode<C>>;
}

impl<'a> LoadSource<&'a str> for &'a str {
    fn load(self) -> Result<SourceCode<&'a str>> {
        Ok(SourceCode::new(Source::default(), self))
    }
}

impl LoadSource<String> for String {
    fn load(self) -> Result<SourceCode<String>> {
        Ok(SourceCode::new(Source::default(), self))
    }
}

impl LoadSource<String> for &Path {
    fn load(self) -> Result<SourceCode<String>> {
        use anyhow_std::PathAnyhow;

        let code = self.read_to_string_anyhow()?;
        let source = Source::from(self);
        Ok(SourceCode::new(source, code))
    }
}

// impl<'a> LoadSource<&'a str> for (&'a Path, &'a str) {
//     fn load(self) -> Result<SourceCode<&'a str>> {
//         let (path, text) = self;
//         let source = Source::from(path);
//         Ok(SourceCode::new(source, text))
//     }
// }
