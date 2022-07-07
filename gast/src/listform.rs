use std::fmt;

/// A general structure for a sequence of items, with an optional tail, used for both list patterns
/// and expressions in the ast, examples: `[]`, `[32]`, `[a, b, ..t]`
#[derive(Clone, Debug, PartialEq)]
pub struct ListForm<Elem, Tail> {
    body: Vec<Elem>,
    tail: Option<Tail>,
}

impl<X, T> ListForm<X, T> {
    pub fn new<I>(body: I, tail: Option<T>) -> Self
    where
        I: IntoIterator<Item = X>,
    {
        ListForm {
            body: body.into_iter().collect(),
            tail,
        }
    }

    pub fn into_reverse_fold<S, TT, F>(self, ttail: TT, f: F) -> S
    where
        TT: FnOnce(Option<T>) -> S,
        F: Fn(S, X) -> S,
    {
        self.body.into_iter().rev().fold(ttail(self.tail), f)
    }

    pub fn try_map<TX, DX, TT, DT, E>(self, telem: TX, ttail: TT) -> Result<ListForm<DX, DT>, E>
    where
        TX: Fn(X) -> Result<DX, E>,
        TT: FnOnce(T) -> Result<DT, E>,
    {
        let bodyres: Result<Vec<DX>, E> = self.body.into_iter().map(telem).collect();

        Ok(ListForm {
            body: bodyres?,
            tail: self.tail.map(ttail).transpose()?,
        })
    }
}

impl<X, T> fmt::Display for ListForm<X, T>
where
    X: fmt::Display,
    T: fmt::Display,
{
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
    use crate::ListForm;
    use test_case::test_case;

    #[test_case([], None => "[]")]
    #[test_case([], Some(42) => "[..42]")]
    #[test_case([2], None => "[2]")]
    #[test_case([2], Some(5) => "[2, ..5]")]
    #[test_case([2, 3], Some(5) => "[2, 3, ..5]")]
    fn display<const K: usize>(body: [u8; K], tail: Option<u8>) -> String {
        ListForm::new(body, tail).to_string()
    }
}
