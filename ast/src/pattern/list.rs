use crate::Pattern;
use std::fmt;
use std::ops::Deref;

#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub struct ListPattern(Vec<Pattern>);

impl FromIterator<Pattern> for ListPattern {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Pattern>,
    {
        ListPattern(iter.into_iter().collect())
    }
}

impl IntoIterator for ListPattern {
    type IntoIter = <Vec<Pattern> as IntoIterator>::IntoIter;
    type Item = Pattern;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Deref for ListPattern {
    type Target = [Pattern];

    fn deref(&self) -> &[Pattern] {
        &self.0[..]
    }
}

impl fmt::Display for ListPattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use sappho_fmtutil::fmt_comma_sep;
        write!(f, "[")?;
        fmt_comma_sep(self.iter(), f)?;
        write!(f, "]")?;
        Ok(())
    }
}
