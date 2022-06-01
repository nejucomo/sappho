use crate::error::{BareError, SourcedError};

use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub struct ErrorSet<E>(Vec<E>);

pub type Errors = ErrorSet<SourcedError>;

impl Errors {
    pub fn attach_source(path: Option<PathBuf>, src: &str, bares: Vec<BareError>) -> Self {
        ErrorSet(
            bares
                .into_iter()
                .map(|bare| SourcedError::new(path.clone(), src, bare))
                .collect(),
        )
    }
}

impl<E> ErrorSet<E> {
    pub fn track_error<T>(&mut self, r: Result<T, E>) -> Option<T> {
        match r {
            Ok(x) => Some(x),
            Err(e) => {
                self.push(e);
                None
            }
        }
    }

    pub fn push(&mut self, error: E) {
        self.0.push(error)
    }

    pub fn extend(&mut self, sub: Self) {
        self.0.extend(sub.0);
    }

    pub fn into_result(self) -> Result<(), Self> {
        if self.0.is_empty() {
            Ok(())
        } else {
            Err(self)
        }
    }
}

impl<E> Default for ErrorSet<E> {
    fn default() -> Self {
        ErrorSet(vec![])
    }
}

impl<T, E> From<T> for ErrorSet<E>
where
    Vec<E>: From<T>,
{
    fn from(x: T) -> Self {
        ErrorSet(Vec::from(x))
    }
}

impl<E> fmt::Display for ErrorSet<E>
where
    E: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for error in self.0.iter() {
            error.fmt(f)?;
        }
        Ok(())
    }
}
