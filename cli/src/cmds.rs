use sappho_ast::PureExpr;
use sappho_parser::LoadParseError;

use crate::{SourceOption, UnparseFormat};

// HACK FIXME: we need a new error type rather than a source-based error.
pub fn repl() -> Result<(), sappho_repl::ReplError> {
    sappho_repl::run_repl()
}

pub fn eval(source: &SourceOption) -> Result<(), sappho_interpreter::Error<'_>> {
    let x = sappho_interpreter::interpret(source)?;
    println!("{}", x);
    Ok(())
}

pub fn parse<'a>(
    source: &'a SourceOption,
    format: &'a UnparseFormat,
) -> Result<(), LoadParseError<'a>> {
    let x = sappho_parser::parse(source)?;
    unparse(x, format);
    Ok(())
}

pub fn fuzz(max_depth: usize, format: &UnparseFormat) {
    let (seed, x) = sappho_ast_fuzz::random_expr(max_depth);
    println!("# AstFuzz seed: {seed}");
    unparse(x, format)
}

fn unparse(x: PureExpr, format: &UnparseFormat) {
    use sappho_transform::{canonicalize, reduce};
    use UnparseFormat::*;

    match format {
        AST => println!("{:#?}", x),
        Direct => println!("{}", x),
        Canonical => println!("{}", canonicalize(x)),
        Reduced => println!("{}", reduce(x)),
    };
}
