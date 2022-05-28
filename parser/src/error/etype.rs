use chumsky::error::Error as ChumskyError;
use chumsky::error::Simple;
use derive_more::From;
use std::fmt;

#[derive(Debug, From)]
pub struct Error(Simple<char>);

pub type Span = <Simple<char> as ChumskyError<char>>::Span;
pub type Label = <Simple<char> as ChumskyError<char>>::Label;

impl Error {
    pub fn custom(span: Span, msg: String) -> Self {
        Error(Simple::custom(span, msg))
    }
}

impl ChumskyError<char> for Error {
    type Span = Span;
    type Label = Label;

    fn expected_input_found<Iter: IntoIterator<Item = Option<char>>>(
        span: Self::Span,
        expected: Iter,
        found: Option<char>,
    ) -> Self {
        Error(Simple::expected_input_found(span, expected, found))
    }

    fn with_label(self, label: Self::Label) -> Self {
        Error(self.0.with_label(label))
    }

    fn merge(self, other: Self) -> Self {
        Error(self.0.merge(other.0))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
