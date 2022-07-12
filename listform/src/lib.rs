use sappho_fmtutil::{DisplayDepth, FmtResult, Formatter};

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

    pub fn map_elems<F, DX>(self, f: F) -> ListForm<DX, T>
    where
        F: Fn(X) -> DX,
    {
        ListForm {
            body: self.body.into_iter().map(f).collect(),
            tail: self.tail,
        }
    }

    pub fn map_tail<F, DT>(self, f: F) -> ListForm<X, DT>
    where
        F: Fn(T) -> DT,
    {
        ListForm {
            body: self.body,
            tail: self.tail.map(f),
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

impl<X, T, E> ListForm<X, Result<T, E>> {
    pub fn transpose_tail(self) -> Result<ListForm<X, T>, E> {
        Ok(ListForm {
            body: self.body,
            tail: self.tail.transpose()?,
        })
    }
}

impl<X, T> DisplayDepth for ListForm<X, T>
where
    X: DisplayDepth,
    T: DisplayDepth,
{
    fn fmt_depth(&self, f: &mut Formatter, depth: usize) -> FmtResult {
        use sappho_fmtutil::indent;

        writeln!(f, "[")?;
        for elem in self.body.iter() {
            indent(f, depth + 1)?;
            elem.fmt_depth(f, depth + 1)?;
            writeln!(f, ",")?;
        }

        if let Some(tail) = &self.tail {
            indent(f, depth + 1)?;
            write!(f, "..")?;
            tail.fmt_depth(f, depth + 1)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl<X, T> std::fmt::Display for ListForm<X, T>
where
    X: DisplayDepth,
    T: DisplayDepth,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.fmt_depth(f, 0)
    }
}

#[cfg(test)]
mod tests {
    use crate::ListForm;
    use sappho_fmtutil::{DisplayDepth, FmtResult, Formatter};
    use test_case::test_case;

    struct X;

    impl DisplayDepth for X {
        fn fmt_depth(&self, f: &mut Formatter, _depth: usize) -> FmtResult {
            write!(f, "X")
        }
    }

    #[test_case([], None => "[]")]
    #[test_case([], Some(X) => "[..X]")]
    #[test_case([X], None => "[X]")]
    #[test_case([X], Some(X) => "[X, ..X]")]
    #[test_case([X, X], Some(X) => "[X, X, ..X]")]
    fn display<const K: usize>(body: [X; K], tail: Option<X>) -> String {
        ListForm::new(body, tail).to_string()
    }
}
