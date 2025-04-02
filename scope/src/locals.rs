use sappho_attrs::Attrs;
use sappho_listform::ListForm;
use sappho_pattern::{BindPattern, Pattern};
use sappho_primval::PrimVal;
use sappho_value::{Valuable, Value};

use crate::{BindError, BindResult};

/// # TODO
///
/// Ensure [Attrs::define] can not fail at runtime due to static analysis of lexical scopes.
#[derive(Debug, Default)]
pub struct Locals(Attrs<Value>);

impl Locals {
    pub fn from_binding(binding: Pattern, value: Value) -> BindResult<Self> {
        let mut myself = Locals::default();
        binding.bind_value(&mut myself, value)?;
        Ok(myself)
    }
}

pub(crate) trait BindValue: Sized {
    fn bind_value(self, locals: &mut Locals, value: Value) -> BindResult<()>;
}

impl BindValue for Pattern {
    fn bind_value(self, locals: &mut Locals, value: Value) -> BindResult<()> {
        use Pattern::*;

        match self {
            Bind(x) => x.bind_value(locals, value),
            LitEq(x) => x.bind_value(locals, value),
            Unpack(x) => x.bind_value(locals, value),
            List(x) => x.bind_value(locals, value),
        }
    }
}

impl BindValue for BindPattern {
    fn bind_value(self, locals: &mut Locals, value: Value) -> BindResult<()> {
        locals.0.define(self, value)?;
        Ok(())
    }
}

impl BindValue for PrimVal {
    fn bind_value(self, _: &mut Locals, value: Value) -> BindResult<()> {
        if Value::from(self) == value {
            Ok(())
        } else {
            Err(BindError::LitEqFailed(self, value))
        }
    }
}

impl BindValue for Attrs<Pattern> {
    fn bind_value(self, locals: &mut Locals, value: Value) -> BindResult<()> {
        for (name, binding) in self {
            let v = value.attr_lookup(&name)?;
            binding.bind_value(locals, v.clone())?;
        }
        Ok(())
    }
}

impl BindValue for ListForm<Pattern, BindPattern> {
    fn bind_value(self, locals: &mut Locals, value: Value) -> BindResult<()> {
        xxx
    }
}
