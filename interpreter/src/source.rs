use pathutil::PathExt;
use std::borrow::Cow;
use std::path::{Path, PathBuf};

pub trait Source {
    fn path(&self) -> Option<&Path>;
    fn load_source<'a>(&'a self) -> std::io::Result<Cow<'a, str>>;
}

impl<P, S> Source for (P, S)
where
    P: AsRef<Path>,
    Cow<'a, str>: From<S>,
{
    fn path(&self) -> Option<&Path> {
        Some(self.0.as_ref())
    }

    fn load_source<'a>(&'a self) -> std::io::Result<Cow<'a, str>> {
        Ok(Cow::from(self.1))
    }
}

impl<P> Source for P
where
    P: AsRef<Path>,
{
    fn path(&self) -> Option<&Path> {
        Some(self.as_ref())
    }

    fn load_source<'a>(&'a self) -> std::io::Result<Cow<'a, str>> {
        self.pe_read_to_string().map(Cow::from)
    }
}
