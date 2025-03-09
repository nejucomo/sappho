use crate::error::{BareError, SourcedError};
use sappho_source::SourceCode;
use std::fmt;

#[derive(Debug)]
pub struct ErrorSet<E>(Vec<E>);

pub type Errors = ErrorSet<SourcedError>;

impl Errors {
    pub fn attach_source<C>(scode: SourceCode<C>, bares: Vec<BareError>) -> Self
    where
        C: Clone + ToString + AsRef<str>,
    {
        ErrorSet(
            bares
                .into_iter()
                .map(|bare| SourcedError::new(scode.clone(), bare))
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

impl<E> From<E> for ErrorSet<E> {
    fn from(e: E) -> Self {
        ErrorSet(vec![e])
    }
}

impl<E> FromIterator<Result<(), E>> for ErrorSet<E> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Result<(), E>>,
    {
        let mut errors = Self::default();
        for res in iter {
            errors.track_error(res);
        }
        errors
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
