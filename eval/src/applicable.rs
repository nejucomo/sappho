use sappho_value::Value;

use crate::scoped::Scoped;

pub(crate) trait Applicable {
    fn apply(self, arg: Value) -> Value;
}

impl Applicable for Scoped<Value> {
    fn apply(self, arg: Value) -> Value {
        fixme
    }
}
