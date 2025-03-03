use std::ops::Deref;

use derive_new::new;

#[derive(Debug, new)]
pub struct CmtPrefixed<T> {
    comment: Option<String>,
    node: T,
}

impl<T> CmtPrefixed<T> {
    pub fn comment(&self) -> Option<&str> {
        self.comment.as_ref().map(String::as_ref)
    }
}

impl<T> Deref for CmtPrefixed<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}
