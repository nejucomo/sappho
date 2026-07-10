use crate::Result;
use sappho_eval::{eval, ValRef};
use sappho_parser::parse;
use sappho_source::LoadSource;
use sappho_transform::reduce;

pub fn interpret<'e, S>(source: S) -> Result<'e, ValRef>
where
    S: LoadSource<'e>,
{
    let ast = parse(source)?;
    let val = eval(reduce(ast))?;
    Ok(val)
}
