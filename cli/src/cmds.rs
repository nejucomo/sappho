use crate::{Result, SourceOption};

pub fn parse(srcopt: &SourceOption) -> Result<()> {
    let source = srcopt.read()?;
    let x = sappho_parser::parse(srcopt.path(), &source)?;
    println!("Parsed: {:#?}", x);
    Ok(())
}

pub fn eval(srcopt: &SourceOption) -> Result<()> {
    let source = srcopt.read()?;
    let ast = sappho_parser::parse(srcopt.path(), &source)?;
    let x = sappho_eval::eval(ast)?;
    println!("{:#?}", x);
    Ok(())
}
