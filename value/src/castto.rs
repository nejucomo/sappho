use crate::PseudoType;

pub trait CastTo<T>
where
    T: PseudoType,
{
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
