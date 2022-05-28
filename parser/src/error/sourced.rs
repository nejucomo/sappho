use crate::error::BareError;
use std::fmt;

#[derive(Debug)]
pub struct SourcedError {
    source: String,
    bare: BareError,
}

impl SourcedError {
    pub fn new(source: &str, bare: BareError) -> Self {
        let source = source.to_string();
        SourcedError { source, bare }
    }
}

impl fmt::Display for SourcedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.bare.fmt(f)?;
        write!(f, "\nIn source: {:?}", self.source)
    }
}
