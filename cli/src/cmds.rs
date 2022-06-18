use crate::{Result, SourceOption};

pub fn parse(source: &SourceOption) -> Result<()> {
    let x = sappho_parser::parse(source)?;
    println!("Parsed: {:#?}", x);
    Ok(())
}

pub fn eval(source: &SourceOption) -> Result<()> {
    let x = sappho_interpreter::interpret(source)?;
    println!("{:#?}", x);
    Ok(())
}
