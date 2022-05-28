use crate::Result;
use std::path::PathBuf;

pub fn parse(path: Option<PathBuf>, source: &str) -> Result<()> {
    let x = sappho_parser::parse(path, source)?;
    println!("Parsed: {:#?}", x);
    Ok(())
}
