use std::fmt;

/// A general structure for a sequence of items, such as a list expression, ie `[x, 42, y]`.
#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub struct ListForm<T>(Vec<T>);

impl<T> ListForm<T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter()
    }
}

impl<T> AsRef<[T]> for ListForm<T> {
    fn as_ref(&self) -> &[T] {
        self.0.as_ref()
    }
}

impl<T> FromIterator<T> for ListForm<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        ListForm(iter.into_iter().collect())
    }
}

impl<T> IntoIterator for ListForm<T> {
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> fmt::Display for ListForm<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use sappho_fmtutil::fmt_comma_sep;

        write!(f, "[")?;
        fmt_comma_sep(&self.0, f)?;
        write!(f, "]")?;
        Ok(())
    }
}
