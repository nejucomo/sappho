use derive_more::From;
use derive_new::new;
use either::Either::{Left, Right};
use sappho_attrs::errors::{Missing, Redefinition};
use sappho_attrs::Attrs;
use sappho_identifier::RcId;
use sappho_list::{List, ListIter};
use sappho_listform::ListForm;
use sappho_pattern::BindPattern;
use sappho_pattern::Pattern;
use sappho_primval::PrimVal;
use thiserror::Error;

use crate::{AsError, VResult, Valuable as _, Value, ValueError};

/// A set of local bindings
///
/// # Language Design
///
/// All names must be declared prior to their references. At the same time, `let` expressions enable mutual recursion, such as two `fn` which call each other in some cases. An example might be two functions used for processing JSON, where one takes any JSON value and processes it, and another takes a JSON list and processes that.
///
/// In order for mutual recursion to work, a `declare` clause must be used. It allows introducing a binding without providing a definition. Later on, it's value can be supplied in a normal `let` clause. In the interrim period between being declared and being defined, a binding may be dereferenced, which is an error.
///
/// Note: this is not due to the `declare` feature; the language could lack the `declare` clause feature, but would still enable mutual recursion via `let` clauses, in which case it would still have to account for dereferencing an undefined binding. The purpose of the `declare` clause is to ensure the rule that "bindings always appear before their use".
///
/// # Implementation Note
///
/// We represent undefined bindings as `None` and defined bindings as `Some(value)`.
#[derive(Clone, Debug, Default)]
pub struct Locals(Attrs<Option<Value>>);

impl Locals {
    pub(crate) fn lookup_opt(&self, id: &RcId) -> Option<Option<&Value>> {
        self.0.get_opt(id).map(|optv| optv.as_ref())
    }

    pub fn declare(&mut self, bindings: &Pattern) -> Result<(), BindError> {
        self.bind_inner(bindings, None)
            .map_err(|t| t.convert_without_value(bindings.clone()))
    }

    pub fn bind(&mut self, bindings: Pattern, value: Value) -> VResult<(), BindError> {
        self.bind_inner(&bindings, Some(&value))
            .map_err(|t| t.convert_with_value(bindings, value))
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
    AttrRedefinition(#[from] Redefinition<Option<Value>>),
    #[error(transparent)]
    AsError(#[from] AsError),
    #[error("does not equal literal match")]
    LitNotEq,
    #[error("unmatched list tail")]
    UnmatchedTail,
}

/// I'm an internal enum for error plumbing
#[derive(From)]
enum ErrorPlumbing {
    #[from(
        BindErrorReason,
        Missing,
        Redefinition<Option<Value>>,
    )]
    Ber(BindErrorReason),
    #[from]
    Vem(ValueError<Missing>),
    #[from]
    Veae(ValueError<AsError>),
    #[from]
    Vebe(ValueError<BindError>),
}

impl ErrorPlumbing {
    fn convert_with_value(self, b: Pattern, v: Value) -> ValueError<BindError> {
        use ErrorPlumbing::*;

        match self {
            Ber(e) => v.wrap_error(BindError::new(b, e)),
            Vem(e) => e.map(|r| BindError::new(b, r)),
            Veae(e) => e.map(|r| BindError::new(b, r)),
            Vebe(e) => e,
        }
    }

    fn convert_without_value(self, b: Pattern) -> BindError {
        use ErrorPlumbing::*;

        match self {
            Ber(e) => BindError::new(b, e),
            Vem(e) => BindError::new(b, e.drop_value()),
            Veae(e) => BindError::new(b, e.drop_value()),
            Vebe(e) => e.drop_value(),
        }
    }
}

trait Bind<P> {
    fn bind_inner(&mut self, b: &P, optv: Option<&Value>) -> Result<(), ErrorPlumbing>;
}

impl Bind<Pattern> for Locals {
    fn bind_inner(
        &mut self,
        bindings: &Pattern,
        optv: Option<&Value>,
    ) -> Result<(), ErrorPlumbing> {
        use Pattern::*;

        match bindings {
            Bind(x) => self.bind_inner(x, optv),
            LitEq(x) => self.bind_inner(x, optv),
            Unpack(x) => self.bind_inner(x, optv),
            List(x) => self.bind_inner(x, optv),
        }
    }
}

impl Bind<BindPattern> for Locals {
    fn bind_inner(&mut self, b: &BindPattern, optv: Option<&Value>) -> Result<(), ErrorPlumbing> {
        let rcid = RcId::from(b.clone());
        self.0.define(rcid, optv.cloned())?;
        Ok(())
    }
}

impl Bind<PrimVal> for Locals {
    fn bind_inner(&mut self, prim: &PrimVal, optv: Option<&Value>) -> Result<(), ErrorPlumbing> {
        if let Some(vref) = optv {
            if &Value::from(*prim) == vref {
                Ok(())
            } else {
                Err(BindErrorReason::LitNotEq.into())
            }
        } else {
            Ok(())
        }
    }
}

impl Bind<Attrs<Pattern>> for Locals {
    fn bind_inner(
        &mut self,
        bindings: &Attrs<Pattern>,
        optv: Option<&Value>,
    ) -> Result<(), ErrorPlumbing> {
        for (name, binding) in bindings.as_refs() {
            let optattrval = optv.map(|v| v.attr_lookup(&name)).transpose()?;

            self.bind_inner(binding, optattrval)?;
        }

        Ok(())
    }
}

impl Bind<ListForm<Pattern, BindPattern>> for Locals {
    fn bind_inner(
        &mut self,
        bindings: &ListForm<Pattern, BindPattern>,
        optv: Option<&Value>,
    ) -> Result<(), ErrorPlumbing> {
        let optlval = optv.map(|lval| lval.as_list()).transpose()?;
        let mut it = OptListIter(optlval.map(|lval| lval.iter()));
        let mut optail = None;
        for (ei, optv) in bindings.iter().zip(it.by_ref()) {
            match ei {
                Left(b) => {
                    self.bind_inner(b, optv)?;
                }
                Right(bindpat) => {
                    assert!(optail.replace(bindpat).is_none());
                }
            }
        }

        let remainder: Option<List<Value>> = it.0.map(|lit| lit.into());
        if let Some(bindpat) = optail {
            self.bind_inner(bindpat, remainder.map(|l| l.into()).as_ref())?;
            Ok(())
        } else if remainder.map(|r| r.length() > 0).unwrap_or(false) {
            Err(BindErrorReason::UnmatchedTail.into())
        } else {
            Ok(())
        }
    }
}

struct OptListIter<'a>(Option<ListIter<'a, Value>>);

impl<'a> Iterator for OptListIter<'a> {
    type Item = Option<&'a Value>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.as_mut().map(|lit| lit.next())
    }
}
