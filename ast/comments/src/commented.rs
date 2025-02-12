use derive_new::new;
use sappho_unparse::Unparse;

/// A wrapper that tracks comments
#[derive(Clone, Debug, PartialEq, new)]
pub struct Commented<T> {
    comment: String,
    item: T,
}

impl<T> Commented<T> {
    pub fn into_inner(self) -> (String, T) {
        (self.comment, self.item)
    }

    pub fn comment(&self) -> &str {
        &self.comment
    }

    pub fn item(&self) -> &T {
        &self.item
    }

    pub fn map<F, U>(self, f: F) -> Commented<U>
    where
        F: FnOnce(T) -> U,
    {
        Commented {
            comment: self.comment,
            item: f(self.item),
        }
    }

    pub fn try_map<F, U, E>(self, f: F) -> Result<Commented<U>, E>
    where
        F: FnOnce(T) -> Result<U, E>,
    {
        self.map(f).transpose()
    }
}

impl<T, E> Commented<Result<T, E>> {
    pub fn transpose(self) -> Result<Commented<T>, E> {
        let (comment, res) = self.into_inner();
        let item = res?;
        Ok(Commented { comment, item })
    }
}

impl<T> From<T> for Commented<T> {
    fn from(item: T) -> Self {
        Commented::new("".to_string(), item)
    }
}

impl<T> Unparse for Commented<T>
where
    T: Unparse,
{
    fn unparse_into(&self, s: &mut sappho_unparse::Stream) {
        use sappho_unparse::Brackets::Parens;
        use sappho_unparse::Break;

        if self.comment.is_empty() {
            self.item.unparse_into(s)
        } else {
            s.bracketed(Parens, |subs| {
                for line in self.comment().lines() {
                    subs.write("# ");
                    subs.write(line);
                    subs.write(&Break::Mandatory);
                }

                subs.write(&self.item)
            });
        }
    }
}
