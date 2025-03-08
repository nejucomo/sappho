use chumsky::Parser as _;
use sappho_source::{LoadSource, SourceCode};

use crate::error::{ChumskyError, Error, ParseError};
use crate::primitive::space;

pub trait Parser<Output>: Sized + chumsky::Parser<char, Output, Error = ChumskyError> {
    fn load_and_parse<L, C>(&self, loadable: L) -> Result<Output, Error>
    where
        L: LoadSource<C>,
        C: AsRef<str>,
    {
        let source = loadable.load().map_err(Error::Load)?;
        let parsed = parse_source(self, source)?;
        Ok(parsed)
    }

    fn then_space(self) -> impl Parser<Output> {
        self.then_ignore(space())
    }

    fn then_opt_space(self) -> impl Parser<Output> {
        self.then_ignore(space().or_not())
    }
}

impl<P, O> Parser<O> for P where P: chumsky::Parser<char, O, Error = ChumskyError> {}

// helper code
fn parse_source<C, P, O>(parser: P, sc: SourceCode<C>) -> Result<O, ParseError>
where
    C: AsRef<str>,
    P: Parser<O>,
{
    use chumsky::primitive::end;
    use chumsky::Parser as _;

    parser
        .then_ignore(end())
        .parse(sc.code())
        .map_err(|errors| ParseError::new(sc.source().clone(), errors))
}
