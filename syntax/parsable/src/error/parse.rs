use std::fmt;

use chumsky::error::Simple;
use sappho_source::Source;

use crate::error::Span;

#[derive(Debug, derive_more::Constructor, thiserror::Error)]
pub struct ParseError {
    origin: Source,
    errors: Vec<ChumskyError>,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        assert!(!self.errors.is_empty());

        write!(f, "Parse errors in {}:", self.origin)?;
        if self.errors.len() == 1 {
            let error = &self.errors[0].0;
            write!(f, " {error}")?;
        } else {
            for (i, ce) in self.errors.iter().enumerate() {
                write!(f, "\n  Error {}: {}", i, ce.0)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, derive_more::From)]
pub struct ChumskyError(Simple<char, Span>);

impl ChumskyError {
    pub fn custom<M: ToString>(span: Span, msg: M) -> Self {
        ChumskyError(Simple::custom(span, msg))
    }
}

impl chumsky::Error<char> for ChumskyError {
    type Span = Span;
    type Label = &'static str;

    fn expected_input_found<Iter: IntoIterator<Item = Option<char>>>(
        span: Self::Span,
        expected: Iter,
        found: Option<char>,
    ) -> Self {
        ChumskyError(Simple::expected_input_found(span, expected, found))
    }

    fn with_label(self, label: Self::Label) -> Self {
        ChumskyError(self.0.with_label(label))
    }

    fn merge(self, other: Self) -> Self {
        ChumskyError(self.0.merge(other.0))
    }
}
