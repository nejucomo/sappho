use chumsky::Parser as _;
use sappho_source::{LoadSource, SourceCode};

use crate::error::{ChumskyError, Error, ParseError};
use crate::primitive::space;
use crate::spanned::Spanned;

pub trait Parser<Output>:
    Sized + Clone + chumsky::Parser<char, Output, Error = ChumskyError>
{
    fn load_and_parse<L, C>(&self, loadable: L) -> Result<Output, Error>
    where
        L: LoadSource<C>,
        C: AsRef<str> + ToString,
    {
        let source = loadable.load().map_err(Error::Load)?;
        let parsed = parse_source(self, source)?;
        Ok(parsed)
    }

    fn spanned(self) -> impl Parser<Spanned<Output>> {
        self.map_with_span(Spanned::new)
    }

    fn try_map_ez<F, O, E>(self, f: F) -> impl Parser<O>
    where
        F: Clone + Fn(Output) -> Result<O, E>,
        E: ToString,
    {
        self.try_map(move |v, span| f(v).map_err(|e| ChumskyError::custom(span, e)))
    }

    fn err_ez<O, E>(self, error: E) -> impl Parser<O>
    where
        E: ToString,
    {
        let errmsg = error.to_string();
        self.try_map_ez::<_, O, _>(move |_| Err(errmsg.clone()))
    }

    fn then_space(self) -> impl Parser<Output> {
        self.then_ignore(space())
    }

    fn then_opt_space(self) -> impl Parser<Output> {
        self.then_ignore(space().or_not())
    }

    fn space_around(self) -> impl Parser<Output> {
        space().ignore_then(self).then_space()
    }

    fn opt_space_around(self) -> impl Parser<Output> {
        space().or_not().ignore_then(self).then_opt_space()
    }
}

impl<P, O> Parser<O> for P where P: chumsky::Parser<char, O, Error = ChumskyError> + Clone {}

// helper code
fn parse_source<C, P, O>(parser: P, sc: SourceCode<C>) -> Result<O, ParseError>
where
    C: AsRef<str> + ToString,
    P: Parser<O>,
{
    use chumsky::primitive::end;
    use chumsky::Parser as _;

    let sc = sc.to_owned();

    parser
        .then_ignore(end())
        .parse(sc.code())
        .map_err(|errors| ParseError::new(sc, errors))
}
