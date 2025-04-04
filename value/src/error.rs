use derive_new::new;
use thiserror::Error;

use crate::Value;

pub type VResult<T, E> = Result<T, ValueError<E>>;

#[derive(Debug, Error, new)]
#[error("{inner};\n  -for value: {value}")]
pub struct ValueError<E> {
    value: Value,
    inner: E,
}

impl<E> ValueError<E> {
    pub fn drop_value(self) -> E {
        self.inner
    }

    pub fn map<F, E2>(self, f: F) -> ValueError<E2>
    where
        F: FnOnce(E) -> E2,
    {
        ValueError {
            value: self.value,
            inner: f(self.inner),
        }
    }
}
