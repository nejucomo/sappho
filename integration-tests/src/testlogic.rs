use std::path::PathBuf;

pub fn test_eval(inpath: PathBuf, input: &str, expected: &str) {
    use sappho_interpreter::interpret;

    let res = interpret((inpath.as_path(), input));
    let actual = match res {
        Ok(x) => x.to_string(),
        Err(x) => x.to_string(),
    };

    check_equal(actual, expected);
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

    check_equal(actual, expected);
}

fn check_equal(expected: String, actual: &str) {
    if expected.trim_end() != actual.trim_end() {
        panic!(
            "Mismatched expectation:\nExpected:\n  {}\nActual:\n  {}\n",
            expected.replace('\n', "\n  "),
            actual.replace('\n', "\n  ")
        );
    }
}
