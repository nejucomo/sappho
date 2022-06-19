use std::path::PathBuf;

pub fn test_eval(inpath: PathBuf, input: &str, expected: &str) {
    use sappho_interpreter::interpret;

    let res = interpret((inpath.as_path(), input));
    let actual = match res {
        Ok(x) => x.to_string(),
        Err(x) => x.to_string(),
    };
    assert_eq!(expected.trim_end(), actual.trim_end());
}

pub fn test_unparse(inpath: PathBuf, input: &str, expected: &str, style: &str) {
    use sappho_east::PureExpr;
    use sappho_parser::parse;

    let ast = parse((inpath.as_path(), input)).unwrap();
    let actual = if style == "canonical" {
        ast.to_string()
    } else {
        PureExpr::from(ast).to_string()
    };

    assert_eq!(expected.trim_end(), actual.trim_end());
}
