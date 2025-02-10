use sappho_ast::PureExpr;

use crate::{Result, SourceOption, UnparseFormat};

pub fn eval(source: &SourceOption) -> Result<()> {
    let x = sappho_interpreter::interpret(source)?;
    println!("{}", x);
    Ok(())
}

pub fn parse<'a>(source: &'a SourceOption, format: &'a UnparseFormat) -> Result<'a, ()> {
    let x = sappho_parser::parse(source)?;
    unparse(x, format)
}

pub fn fuzz(max_depth: usize, format: &UnparseFormat) -> Result<()> {
    let (seed, x) = sappho_ast_fuzz::random_expr(max_depth);
    println!("# AstFuzz seed: {seed}");
    unparse(x, format)
}

fn unparse(x: PureExpr, format: &UnparseFormat) -> Result<()> {
    use sappho_transform::{canonicalize, reduce};
    use UnparseFormat::*;

    match format {
        AST => println!("{:#?}", x),
        Direct => println!("{}", x),
        Canonical => println!("{}", canonicalize(x)),
        Reduced => println!("{}", reduce(x)),
    };
    Ok(())
}
