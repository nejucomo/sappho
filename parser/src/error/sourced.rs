use crate::error::BareError;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub struct SourcedError {
    path: Option<PathBuf>,
    source: String,
    bare: BareError,
}

impl SourcedError {
    pub fn new(path: Option<PathBuf>, source: &str, bare: BareError) -> Self {
        let source = source.to_string();
        SourcedError { path, source, bare }
    }
}

impl fmt::Display for SourcedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.bare.fmt(f)?;
        write!(
            f,
            "\nIn source {}:\n{:?}",
            self.path
                .as_ref()
                .map(|p| format!("{:?}", p.display()))
                .unwrap_or_else(|| "<string>".to_string()),
            &self.source
        )
    }
}
