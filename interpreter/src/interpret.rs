use crate::Result;
use sappho_eval::{eval, ValRef};
use sappho_parser::parse;
use sappho_source::LoadSource;
use sappho_transform::reduce;

pub fn interpret<'a, S>(source: S) -> Result<'a, ValRef>
where
    S: LoadSource<'a>,
{
    let ast = parse(source)?;
    let val = eval(reduce(ast))?;
    Ok(val)
}
