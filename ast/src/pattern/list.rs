use crate::{Identifier, Pattern};
use std::fmt;

#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub struct ListPattern {
    pub body: Vec<Pattern>,
    pub tail: Option<Identifier>,
}

impl ListPattern {
    pub fn new<I>(body: I, tail: Option<Identifier>) -> Self
    where
        I: IntoIterator<Item = Pattern>,
    {
        ListPattern {
            body: body.into_iter().collect(),
            tail,
        }
    }
}

impl fmt::Display for ListPattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use sappho_fmtutil::fmt_comma_sep;
        write!(f, "[")?;
        fmt_comma_sep(self.body.iter(), f)?;
        if let Some(tail) = &self.tail {
            if !self.body.is_empty() {
                write!(f, ", ")?;
            }
            write!(f, "..{}", tail)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{ListPattern, Pattern};
    use test_case::test_case;

    #[test_case([], None => "[]")]
    #[test_case([], Some("t") => "[..t]")]
    #[test_case(["h"], None => "[h]")]
    #[test_case(["h"], Some("t") => "[h, ..t]")]
    #[test_case(["a", "b"], Some("t") => "[a, b, ..t]")]
    fn display<const K: usize>(body: [&str; K], tail: Option<&str>) -> String {
        ListPattern::new(
            body.map(|s| Pattern::Bind(s.to_string())),
            tail.map(|s| s.to_string()),
        )
        .to_string()
    }
}
