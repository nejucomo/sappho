use sappho_source::{LoadSource, SourceCodeLink};
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Default)]
pub enum SourceOption {
    #[default]
    Stdin,
    Path(PathBuf),
}
use SourceOption::*;

impl LoadSource for &SourceOption {
    fn load(self) -> anyhow::Result<SourceCodeLink> {
        match self {
            Stdin => std::io::stdin().load(),
            Path(p) => p.as_path().load(),
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
