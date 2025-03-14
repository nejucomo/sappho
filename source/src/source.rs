use std::fmt;
use std::path::{Path, PathBuf};

use derive_more::From;

use self::Source::*;

/// The origin of a [SourceCode](crate::SourceCode)
#[derive(Clone, Debug, From)]
pub enum Source {
    /// A string in memory
    Memory,
    /// The `stdin` file descriptor
    Stdin,
    /// A filesystem path
    #[from(&Path, PathBuf)]
    FS(PathBuf),
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Memory => write!(f, "<in-memory>"),
            Stdin => write!(f, "<stdin>"),
            FS(pb) => write!(f, "{:?}", pb.display()),
        }
    }
}
