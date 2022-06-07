use sappho_eval::{eval, ValRef};
use sappho_parser::parse;
use std::path::PathBuf;

pub fn test_eval(inpath: PathBuf, input: &str, expected: &str) {
    let res = parse_and_eval(inpath, input);
    let actual = format!("{:#?}", res);
    assert_eq!(expected.trim_end(), &actual);
}

#[derive(Debug, derive_more::From)]
enum Error {
    Parse(sappho_parser::Errors),
    Eval(sappho_eval::Error),
}

fn parse_and_eval(path: PathBuf, input: &str) -> Result<ValRef, Error> {
    let ast = parse(Some(path), input)?;
    let val = eval(ast)?;
    Ok(val)
}
