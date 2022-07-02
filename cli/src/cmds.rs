use crate::{ParseFormat, Result, SourceOption};

pub fn eval(source: &SourceOption) -> Result<()> {
    let x = sappho_interpreter::interpret(source)?;
    println!("{}", x);
    Ok(())
}

pub fn parse<'a>(source: &'a SourceOption, format: &'a ParseFormat) -> Result<'a, ()> {
    use sappho_transform::{canonicalize, reduce};
    use ParseFormat::*;

    let x = sappho_parser::parse(source)?;
    match format {
        AST => println!("{:#?}", x),
        Direct => println!("{}", x),
        Canonical => println!("{}", canonicalize(x)),
        Elemental => println!("{}", reduce(x)),
    };
    Ok(())
}
