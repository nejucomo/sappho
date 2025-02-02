use std::path::PathBuf;

pub fn test_eval(inpath: PathBuf, input: &str, expected: &str) {
    use sappho_interpreter::interpret;

    dbg!(input);
    dbg!(sappho_parser::parse((inpath.as_path(), input)).unwrap());

    let res = interpret((inpath.as_path(), input));
    let actual = match res {
        Ok(x) => x.to_string(),
        Err(x) => x.to_string(),
    };

    check_equal(expected, actual);
}

pub fn test_unparse(inpath: PathBuf, input: &str, expected: &str, style: &str) {
    use sappho_parser::parse;
    use sappho_transform::{canonicalize, reduce};

    let ast = parse((inpath.as_path(), input)).unwrap();
    let actual = if style == "canonical" {
        canonicalize(ast).to_string()
    } else if style == "reduced" {
        reduce(ast).to_string()
    } else {
        panic!("Unknown unparse style {:?}", style);
    };

    check_equal(expected, actual);
}

fn check_equal(expected: &str, actual: String) {
    if expected.trim_end() != actual.trim_end() {
        panic!(
            "Mismatched expectation:\nExpected:\n  {}\nActual:\n  {}\n",
            expected.replace('\n', "\n  "),
            actual.replace('\n', "\n  ")
        );
    }
}
