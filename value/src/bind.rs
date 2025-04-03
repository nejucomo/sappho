use derive_new::new;
use sappho_attrs::{Attrs, Missing};
use sappho_identifier::RcId;
use sappho_pattern::{BindPattern, Pattern};
use sappho_primval::PrimVal;
use thiserror::Error;

use crate::{Locals, Valuable as _, Value, ValueError};

pub(crate) trait Bind<P, V, E = BindErrorReason> {
    fn bind(&mut self, bindings: P, value: V) -> Result<(), E>;
}

#[derive(Debug, Error, new)]
#[error("{reason};\n  -for bindings {bindings:?}")]
pub struct BindError {
    bindings: Pattern,
    reason: BindErrorReason,
}

#[derive(Debug, Error, new)]
pub enum BindErrorReason {
    #[error(transparent)]
    MissingAttr(#[from] Missing),
    #[error("does not equal literal match")]
    LitNotEq,
}

impl Bind<Pattern, Value, ValueError<BindError>> for Locals {
    fn bind(&mut self, bindings: Pattern, value: Value) -> Result<(), ValueError<BindError>> {
        self.bind(&bindings, &value)
            .map_err(|reason| value.wrap_error(BindError::new(bindings, reason)))
    }
}

impl Bind<&Pattern, &Value> for Locals {
    fn bind(&mut self, bindings: &Pattern, value: &Value) -> Result<(), BindErrorReason> {
        use Pattern::*;

        match bindings {
            Bind(x) => self.bind(x, value),
            LitEq(x) => self.bind(x, value),
            Unpack(x) => self.bind(x, value),
            List(x) => self.bind(x, value),
        }
    }
}

impl Bind<&BindPattern, &Value> for Locals {
    fn bind(&mut self, b: &BindPattern, value: &Value) -> Result<(), BindErrorReason> {
        let rcid = RcId::from(b.clone());
        self.define(rcid, value.clone())?;
        Ok(())
    }
}

impl Bind<&PrimVal, &Value> for Locals {
    fn bind(&mut self, prim: &PrimVal, value: &Value) -> Result<(), BindErrorReason> {
        if &Value::from(*prim) == value {
            Ok(())
        } else {
            Err(BindErrorReason::LitNotEq)
        }
    }
}

impl Bind<&Attrs<Pattern>, &Value> for Locals {
    fn bind(&mut self, bindings: &Attrs<Pattern>, value: &Value) -> Result<(), BindErrorReason> {
        for (name, binding) in bindings.as_refs() {
            let attval = value.attr_lookup(&name)?;
            self.bind(binding, attval)?;
        }

        Ok(())
    }
}
