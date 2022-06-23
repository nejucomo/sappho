use crate::{ParseFormat, Result, SourceOption};

pub fn eval(source: &SourceOption) -> Result<()> {
    let x = sappho_interpreter::interpret(source)?;
    println!("{}", x);
    Ok(())
}

pub fn parse<'a>(source: &'a SourceOption, format: &'a ParseFormat) -> Result<'a, ()> {
    let x = sappho_parser::parse(source)?;
    match format {
        ParseFormat::AST => println!("{:#?}", x),
        ParseFormat::Canonical => println!("{}", x),
        ParseFormat::Elemental => println!("{}", sappho_east::PureExpr::from(x)),
    };
    Ok(())
}
