use std::fmt;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use derive_more::From;

use crate::{SourceCode, SourceCodeLink};

use self::Source::*;

/// The origin of a [SourceCode](crate::SourceCode)
///
/// The `FromStr` and `Display` impls are compatible with `clap` such that this type can be a cli option with this syntax:
///
/// - `-` indicates `stdin` (which is also the default)
/// - `(` _expr_ `)` is a literal expression, always surrounded by parentheses
/// - any other string is a filesystem path
#[derive(Debug, From)]
pub enum Source {
    /// A literal expression, surrounded by `(` and `)`
    #[from(&str, String)]
    Literal(String),
    /// The `stdin` file descriptor, given as `-`
    Stdin(std::io::Stdin),
    /// A filesystem path
    #[from(&Path, PathBuf)]
    FS(PathBuf),
    /// A test file path and contents
    #[doc(hidden)]
    TestFile(PathBuf, String),
}

impl Default for Source {
    fn default() -> Self {
        Stdin(std::io::stdin())
    }
}

impl Source {
    /// Load this source
    pub fn load(self) -> std::io::Result<SourceCodeLink> {
        self.load_unlinked().map(SourceCodeLink::from)
    }

    fn load_unlinked(self) -> std::io::Result<SourceCode> {
        let description = self.describe();

        let code = match self {
            Literal(code) => code,
            Stdin(stdin) => std::io::read_to_string(stdin)?,
            FS(pb) => {
                let f = std::fs::File::open(&pb)?;
                std::io::read_to_string(f)?
            }
            TestFile(_, code) => code,
        };

        Ok(SourceCode::new(code, description))
    }

    /// Provide a user-friendly description of this source
    pub fn describe(&self) -> String {
        match self {
            Literal(_) => "<in-memory>".to_string(),
            Stdin(_) => "<stdin>".to_string(),
            FS(pb) => format!("path {:?}", pb.display()),
            TestFile(pb, _) => format!("test path {:?}", pb.display()),
        }
    }
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal(_) => write!(f, "(...)"),
            Stdin(_) => write!(f, "-"),
            FS(pb) => write!(f, "{}", pb.display()),
            //
            TestFile(_, _) => {
                unimplemented!("TestFile sources have no command-line representation: {self:?}")
            }
        }
    }
}

impl FromStr for Source {
    type Err = <PathBuf as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "-" {
            Ok(Stdin(std::io::stdin()))
        } else if s.starts_with('(') && s.ends_with(')') {
            Ok(Literal(s.to_string()))
        } else {
            PathBuf::from_str(s).map(FS)
        }
    }
}
