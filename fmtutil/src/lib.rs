use std::fmt;

pub fn fmt_comma_sep<I, X>(ii: I, f: &mut fmt::Formatter) -> fmt::Result
where
    I: IntoIterator<Item = X>,
    X: fmt::Display,
{
    let mut first = true;
    for x in ii {
        if first {
            first = false;
        } else {
            write!(f, ", ")?;
        }
        x.fmt(f)?;
    }
    Ok(())
}
