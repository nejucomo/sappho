use derive_more::From;
use derive_new::new;
use either::Either::{Left, Right};
use sappho_attrs::errors::{Missing, Redefinition};
use sappho_attrs::Attrs;
use sappho_identifier::RcId;
use sappho_list::List;
use sappho_listform::ListForm;
use sappho_pattern::{BindPattern, Pattern};
use sappho_primval::PrimVal;
use thiserror::Error;

use crate::{Locals, PseudoTypeError, VResult, Valuable, Value, ValueError};

pub trait Bind {
    fn bind(&mut self, bindings: Pattern, value: Value) -> VResult<(), BindError>;
}

impl Bind for Locals {
    fn bind(&mut self, bindings: Pattern, value: Value) -> VResult<(), BindError> {
        self.bind_inner(&bindings, &value)
            .map_err(|t| t.convert(bindings, value))
    }
}

#[derive(Debug, Error, new)]
#[error("{reason};\n  -for bindings {bindings:?}")]
pub struct BindError {
    #[new(into)]
    bindings: Pattern,
    #[new(into)]
    reason: BindErrorReason,
}

#[derive(Debug, Error, new)]
pub enum BindErrorReason {
    #[error(transparent)]
    AttrMissing(#[from] Missing),
    #[error(transparent)]
    AttrRedefinition(#[from] Redefinition<Value>),
    #[error(transparent)]
    PseudoTypeError(#[from] PseudoTypeError),
    #[error("does not equal literal match")]
    LitNotEq,
    #[error("unmatched list tail")]
    UnmatchedTail,
}

/// I'm an internal enum for error plumbing
#[derive(From)]
enum Transport {
    #[from(
        BindErrorReason,
        Missing,
        Redefinition<Value>,
    )]
    Ber(BindErrorReason),
    #[from]
    Vem(ValueError<Missing>),
    #[from]
    Vepte(ValueError<PseudoTypeError>),
    #[from]
    Vebe(ValueError<BindError>),
}

impl Transport {
    fn convert(self, b: Pattern, v: Value) -> ValueError<BindError> {
        use Transport::*;

        match self {
            Ber(e) => v.wrap_error(BindError::new(b, e)),
            Vem(e) => e.map(|r| BindError::new(b, r)),
            Vepte(e) => e.map(|r| BindError::new(b, r)),
            Vebe(e) => e,
        }
    }
}

trait BindInner<P> {
    fn bind_inner(&mut self, b: &P, v: &Value) -> Result<(), Transport>;
}

impl BindInner<Pattern> for Locals {
    fn bind_inner(&mut self, bindings: &Pattern, value: &Value) -> Result<(), Transport> {
        use Pattern::*;

        match bindings {
            Bind(x) => self.bind_inner(x, value),
            LitEq(x) => self.bind_inner(x, value),
            Unpack(x) => self.bind_inner(x, value),
            List(x) => self.bind_inner(x, value),
        }
    }
}

impl BindInner<BindPattern> for Locals {
    fn bind_inner(&mut self, b: &BindPattern, value: &Value) -> Result<(), Transport> {
        let rcid = RcId::from(b.clone());
        self.define(rcid, value.clone())?;
        Ok(())
    }
}

impl BindInner<PrimVal> for Locals {
    fn bind_inner(&mut self, prim: &PrimVal, value: &Value) -> Result<(), Transport> {
        if &Value::from(*prim) == value {
            Ok(())
        } else {
            Err(BindErrorReason::LitNotEq.into())
        }
    }
}

impl BindInner<Attrs<Pattern>> for Locals {
    fn bind_inner(&mut self, bindings: &Attrs<Pattern>, value: &Value) -> Result<(), Transport> {
        for (name, binding) in bindings.as_refs() {
            let attrval = value.attr_lookup(&name)?;

            self.bind_inner(binding, attrval)?;
        }

        Ok(())
    }
}

impl BindInner<ListForm<Pattern, BindPattern>> for Locals {
    fn bind_inner(
        &mut self,
        bindings: &ListForm<Pattern, BindPattern>,
        value: &Value,
    ) -> Result<(), Transport> {
        let lval: &List<Value> = value.cast()?;
        let mut it = lval.iter();
        let mut optail = None;
        for (ei, v) in bindings.iter().zip(it.by_ref()) {
            match ei {
                Left(b) => {
                    self.bind_inner(b, v)?;
                }
                Right(bindpat) => {
                    assert!(optail.replace(bindpat).is_none());
                }
            }
        }

        let remainder: List<Value> = it.into();
        if let Some(bindpat) = optail {
            self.bind(bindpat.clone().into(), remainder.into())?;
            Ok(())
        } else if remainder.length() > 0 {
            Err(BindErrorReason::UnmatchedTail.into())
        } else {
            Ok(())
        }
    }
}
