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
