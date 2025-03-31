use sappho_source::{Source, SourceCodeLink};

use crate::error::{Error, ParseError};
use crate::{ParsableWith, Parser as _};

pub fn load_and_parse<P, S>(source: S) -> Result<P, Error>
where
    P: for<'a> ParsableWith<Option<&'a SourceCodeLink>>,
    Source: From<S>,
{
    let sc = Source::from(source).load().map_err(Error::Load)?;
    let parsed = parse_with_sourcecode(&sc)?;
    Ok(parsed)
}

fn parse_with_sourcecode<'a, P>(sc: &'a SourceCodeLink) -> Result<P, ParseError>
where
    P: ParsableWith<Option<&'a SourceCodeLink>>,
{
    P::parser_with(Some(sc)).parse_sourcecode(sc)
}
