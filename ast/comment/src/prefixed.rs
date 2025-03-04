use std::ops::Deref;

use derive_new::new;
use sappho_unparse::Unparse;

#[derive(Clone, Debug, PartialEq, Eq, new, derive_more::From, derive_more::Into)]
pub struct CmtPrefixed<T> {
    comment: Option<String>,
    node: T,
}

impl<T> CmtPrefixed<T> {
    pub fn comment(&self) -> Option<&str> {
        self.comment.as_ref().map(String::as_ref)
    }

    pub fn map_node<F, U>(self, f: F) -> CmtPrefixed<U>
    where
        F: FnOnce(T) -> U,
    {
        let CmtPrefixed { comment, node } = self;

        let node = f(node);

        CmtPrefixed { comment, node }
    }

    pub fn append_comment_section<S, B>(self, section: S, body: B) -> Self
    where
        S: AsRef<str>,
        B: AsRef<str>,
    {
        let section = section.as_ref();
        let body = body.as_ref();
        self.append_comment(format!("# {section}\n\n{body}\n"))
    }

    pub fn append_comment<S>(mut self, comment: S) -> Self
    where
        S: AsRef<str>,
    {
        let s = self.comment.get_or_insert_default();
        s.push_str(comment.as_ref());
        self
    }
}

impl<T, E> CmtPrefixed<Result<T, E>> {
    pub fn transpose(self) -> Result<CmtPrefixed<T>, E> {
        let CmtPrefixed { comment, node } = self;

        let node = node?;

        Ok(CmtPrefixed { comment, node })
    }
}

impl<T> Deref for CmtPrefixed<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl<T> Unparse for CmtPrefixed<T>
where
    T: Unparse,
{
    fn unparse_into(&self, s: &mut sappho_unparse::Stream) {
        if let Some(cmt) = self.comment() {
            for line in cmt.lines() {
                s.write("# ");
                s.write(line);
                s.write("\n");
            }
        }
        s.write(self.deref())
    }
}
