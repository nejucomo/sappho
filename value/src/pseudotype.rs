use thiserror::Error;

#[derive(Debug, Error)]
#[error("unexpected type, expected <{0}>")]
pub struct PseudoTypeError(&'static str);

pub trait PseudoType {
    /// The user-facing pseudo-type name of `Self`
    fn pseudo_type_name() -> &'static str;

    /// The [TypeError] for `Self`
    fn pseudo_type_error() -> PseudoTypeError {
        PseudoTypeError(Self::pseudo_type_name())
    }
}
