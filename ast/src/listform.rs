use std::fmt;

#[derive(Debug, PartialEq, derive_more::From)]
pub struct ListForm<T>(Vec<T>);

impl<T> ListForm<T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter()
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
        let mut first = true;
        write!(f, "[")?;
        for child in self.iter() {
            if first {
                first = false;
            } else {
                write!(f, ", ")?;
            }
            child.fmt(f)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}
