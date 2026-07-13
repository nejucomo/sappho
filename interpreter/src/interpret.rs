use sappho_code_origin::CodeOrigin;
use sappho_eval::{eval, ValRef};
use sappho_parser::parse;
use sappho_transform::reduce;

use crate::Result;

pub fn interpret<'a>(codor: CodeOrigin<'a>) -> Result<'a, ValRef> {
    let ast = parse(codor)?;
    let val = eval(reduce(ast))?;
    Ok(val)
}
