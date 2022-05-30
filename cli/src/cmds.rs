use crate::{Result, SourceOption};

pub fn parse(srcopt: &SourceOption) -> Result<()> {
    let source = srcopt.read()?;
    let x = sappho_parser::parse(srcopt.path(), &source)?;
    println!("Parsed: {:#?}", x);
    Ok(())
}
