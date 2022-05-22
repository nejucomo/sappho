use crate::error::ErrorSet;
use derive_more::From;
use saplang_ast::PureExpr;
use std::fmt;

pub type Errors = ErrorSet<Error>;

#[derive(Debug)]
pub struct Error(pub std::path::PathBuf, pub Reason);

#[derive(Debug, From)]
pub enum Reason {
    MissingFile(&'static str),
    StrUtf8(std::str::Utf8Error),
    StringUtf8(std::string::FromUtf8Error),
    Parse(crate::Errors),
    InvalidParse(PureExpr),
    MismatchedOutput(Mismatch),
}

#[derive(Debug)]
pub struct Mismatch {
    pub expected: String,
    pub found: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Error(path, reason) = self;

        write!(f, "Error in {:?}:\n{}", path.display(), reason)
    }
}

impl fmt::Display for Reason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Reason::*;

        match self {
            MissingFile(x) => write!(f, "missing file: {}", x),
            StrUtf8(x) => write!(f, "utf8 decode error: {}", x),
            StringUtf8(x) => write!(f, "uft8 decode error: {}", x),
            Parse(x) => write!(f, "parse error: {}", x),
            InvalidParse(x) => write!(f, "unexpected parse: {}", x),
            MismatchedOutput(x) => write!(f, "mismatched output: {}", x),
        }
    }
}

impl fmt::Display for Mismatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "expected:\n{}\nfound:\n{}",
            indent_lines(&self.expected),
            indent_lines(&self.found)
        )
    }
}

fn indent_lines(s: &str) -> String {
    let mut result = String::new();
    for line in s.lines() {
        result += "  ";
        result += line;
        result += "\n";
    }
    result
}
