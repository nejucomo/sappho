use derive_more::From;
use std::fmt;

#[derive(Debug, From)]
pub struct ErrorSet<T>(Vec<T>);

pub type Errors = ErrorSet<Error>;
pub type Error = chumsky::error::Simple<char>;

impl<T> ErrorSet<T> {
    pub fn push(&mut self, error: T) {
        self.0.push(error)
    }

    pub fn into_result(self) -> Result<(), Self> {
        if self.0.is_empty() {
            Ok(())
        } else {
            Err(self)
        }
    }
}

impl<T> Default for ErrorSet<T> {
    fn default() -> Self {
        ErrorSet(vec![])
    }
}

impl<T> fmt::Display for ErrorSet<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for error in self.0.iter() {
            write!(f, "\n{}\n", error)?;
        }
        Ok(())
    }
}
