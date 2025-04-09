use crate::{PseudoType, PseudoTypeError};

pub trait CastTo<T>
where
    T: PseudoType,
{
    fn cast(&self) -> Result<&T, PseudoTypeError> {
        self.cast_opt().ok_or(T::pseudo_type_error())
    }

    fn cast_opt(&self) -> Option<&T> {
        None
    }
}

impl<T> CastTo<T> for T
where
    T: PseudoType,
{
    fn cast_opt(&self) -> Option<&Self> {
        Some(self)
    }
}
