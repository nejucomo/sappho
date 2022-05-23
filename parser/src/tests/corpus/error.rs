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
    InvalidRegex(regex::Error),
}

#[derive(Debug)]
pub struct Mismatch {
    pub expected: regex::Regex,
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
            MismatchedOutput(x) => write!(f, "mismatched output:\n{}", x),
            InvalidRegex(x) => write!(f, "invalid regex: {}", x),
        }
    }
}

impl fmt::Display for Mismatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "+ expected:\n{}\n+ found:\n{}\n+ expected debug: {:?}\n+ found debug: {:?}",
            prefix_lines("| ", self.expected.as_str()),
            prefix_lines("| ", &self.found),
            self.expected.as_str(),
            &self.found,
        )
    }
}

fn prefix_lines(prefix: &str, s: &str) -> String {
    let mut result = String::new();
    for line in s.lines() {
        result += prefix;
        result += line;
        result += "\n";
    }
    result
}
