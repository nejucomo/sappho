use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug)]
pub enum SourceOption {
    Stdin,
    Path(PathBuf),
}

impl SourceOption {
    pub fn read(&self) -> std::io::Result<String> {
        use SourceOption::*;

        match self {
            Stdin => {
                use std::io::Read;
                let mut s = String::new();
                std::io::stdin().read_to_string(&mut s)?;
                Ok(s)
            }
            Path(p) => std::fs::read_to_string(p),
        }
    }

    pub fn path(&self) -> Option<PathBuf> {
        use SourceOption::*;

        match self {
            Stdin => None,
            Path(p) => Some(p.clone()),
        }
    }
}

impl Default for SourceOption {
    fn default() -> Self {
        SourceOption::Stdin
    }
}

impl fmt::Display for SourceOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use SourceOption::*;

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
