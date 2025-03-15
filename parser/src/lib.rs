use sappho_parsable::error::Error;
use sappho_parsable::load_and_parse;
use sappho_source::Source;
use sappho_syntax::PureExpr;

pub fn parse<S>(source: S) -> Result<PureExpr, Error>
where
    Source: From<S>,
{
    load_and_parse(source)
}

#[cfg(test)]
mod tests;
