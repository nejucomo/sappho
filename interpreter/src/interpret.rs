use sappho_eval::{eval, ValRef};
use sappho_parser::parse;
use sappho_source::LoadSource;
use sappho_transform::reduce;

use crate::Result;

pub fn interpret<S>(source: S) -> Result<ValRef>
where
    S: LoadSource,
{
    let ast = parse(source)?;
    let val = eval(reduce(ast))?;
    Ok(val)
}
