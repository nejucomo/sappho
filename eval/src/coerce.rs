use crate::Value;

pub trait Coerce {
    fn coerce_from_value(v: &Value) -> Option<&Self>;
}
