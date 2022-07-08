use std::fmt;

pub fn fmt_comma_sep<I, X>(ii: I, f: &mut fmt::Formatter) -> fmt::Result
where
    I: IntoIterator<Item = X>,
    X: fmt::Display,
{
    let mut ct = CommaTracker::default();
    for x in ii {
        ct.insert(f)?;
        x.fmt(f)?;
    }
    Ok(())
}

#[derive(Debug, Default)]
pub struct CommaTracker(bool);

impl CommaTracker {
    pub fn insert(&mut self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0 {
            write!(f, ", ")
        } else {
            self.0 = true;
            Ok(())
        }
    }
}
