use sappho_ast::PureExpr;

use crate::{CliResult, SourceOption, UnparseFormat};

// HACK FIXME: we need a new error type rather than a source-based error.
pub fn repl() -> Result<(), sappho_repl::ReplError> {
    sappho_repl::run_repl()
}

pub fn eval(source: &SourceOption) -> CliResult<'_, ()> {
    let codor = source.try_load_code()?;
    let x = sappho_interpreter::interpret(codor)?;
    println!("{}", x);
    Ok(())
}

pub fn parse<'a>(source: &'a SourceOption, format: &'a UnparseFormat) -> CliResult<'a, ()> {
    let codor = source.try_load_code()?;
    let x = sappho_parser::parse(codor)?;
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
