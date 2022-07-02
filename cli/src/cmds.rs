use crate::{ParseFormat, Result, SourceOption};

pub fn eval(source: &SourceOption) -> Result<()> {
    let x = sappho_interpreter::interpret(source)?;
    println!("{}", x);
    Ok(())
}

pub fn parse<'a>(source: &'a SourceOption, format: &'a ParseFormat) -> Result<'a, ()> {
    use sappho_ast::PureExpr as APE;
    use sappho_east::PureExpr as EPE;
    use ParseFormat::*;

    let x = sappho_parser::parse(source)?;
    match format {
        AST => println!("{:#?}", x),
        Direct => println!("{}", x),
        Canonical => println!("{}", APE::from(EPE::from(x))),
        Elemental => println!("{}", EPE::from(x)),
    };
    Ok(())
}
