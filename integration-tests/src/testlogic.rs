use sappho_interpreter::interpret;
use std::path::PathBuf;

pub fn test_eval(inpath: PathBuf, input: &str, expected: &str) {
    let res = interpret((inpath.as_path(), input));
    let actual = match res {
        Ok(x) => x.to_string(),
        Err(x) => x.to_string(),
    };
    assert_eq!(expected.trim_end(), actual.trim_end());
}
