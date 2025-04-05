use sappho_attrs::errors::Redefinition;
use thiserror::Error;

use crate::IxRef;

pub type AnnotationResult<T> = Result<T, AnnotationError>;

#[derive(Debug, Error)]
pub enum AnnotationError {
    #[error("duplicate binding: {0}")]
    BindingRedefined(#[from] Redefinition<IxRef>),
}
