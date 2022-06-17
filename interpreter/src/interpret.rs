use crate::Result;
use sappho_eval::{eval, ValRef};
use sappho_parser::parse;
use std::path::PathBuf;

pub fn interpret(path: Option<PathBuf>, code: &str) -> Result<ValRef> {
    let ast = parse(path, code)?;
    let val = eval(ast)?;
    Ok(val)
}
