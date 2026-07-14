use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

use sappho_code_origin::CodeOrigin;

#[derive(Debug, Default)]
pub enum SourceOption {
    #[default]
    Stdin,
    Path(PathBuf),
}
use SourceOption::*;

impl SourceOption {
    pub fn try_load_code(&self) -> anyhow::Result<CodeOrigin<'static>> {
        use SourceOption::*;

        match self {
            Stdin => {
                use std::io::{stdin, Read as _};

                let mut code = "".to_string();
                let n = stdin().read_to_string(&mut code)?;
                assert_eq!(n, code.len());

                Ok(CodeOrigin::new(code, "<stdin>"))
            }
            Path(path) => CodeOrigin::load_path(path),
        }
    }
}

impl fmt::Display for SourceOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stdin => write!(f, "-"),
            Path(p) => write!(f, "{}", p.display()),
        }
    }
}

impl FromStr for SourceOption {
    type Err = <PathBuf as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SourceOption::*;

        if s == "-" {
            Ok(Stdin)
        } else {
            PathBuf::from_str(s).map(Path)
        }
    }
}
