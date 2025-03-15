use sappho_ast_rich::PureExpr;
use sappho_parsable::error::Error;
use sappho_parsable::ParsableWith;
use sappho_source::Source;

pub fn parse<S>(source: S) -> Result<PureExpr, Error>
where
    Source: From<S>,
{
    PureExpr::load_and_parse(source)
}

#[cfg(test)]
mod tests;
