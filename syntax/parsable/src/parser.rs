use sappho_source::{LoadSource, SourceCode};

use crate::error::{ChumskyError, Error, ParseError};

pub trait Parser<Output>: Sized + chumsky::Parser<char, Output, Error = ChumskyError> {
    fn load_parse_source<L, C>(&self, loadable: L) -> Result<Output, Error>
    where
        L: LoadSource<C>,
        C: AsRef<str>,
    {
        let source = loadable.load().map_err(Error::Load)?;
        let parsed = self.parse_source(source)?;
        Ok(parsed)
    }

    fn parse_source<C>(&self, sc: SourceCode<C>) -> Result<Output, ParseError>
    where
        C: AsRef<str>,
    {
        self.parse(sc.code())
            .map_err(|errors| ParseError::new(sc.source().clone(), errors))
    }
}

impl<P, O> Parser<O> for P where P: chumsky::Parser<char, O, Error = ChumskyError> {}
