mod bindfailure;

use crate::{Attrs, Unbound, UnboundKind::Unfulfilled, ValRef};
use sappho_ast_reduced::{Literal, Pattern, UnpackPattern};
use sappho_attrs::{Attrs, IdentRef};
use std::cell::RefCell;

pub use self::bindfailure::{BindFailure, BindFailureReason};

/// A `Frame` maps in-scope bindings to [Option]<[ValRef]> where `None` indicates a
/// forward-reference is not yet fulfilled, while `Some(v)` provides a defined value.
#[derive(Debug, Default)]
pub struct Frame(Attrs<RefCell<Option<ValRef>>>);

impl Frame {
    pub fn declare(&mut self, pattern: &Pattern) {
        use Pattern::*;

        match pattern {
            // BUG: Different patterns in a let may bind the same identifier:
            Bind(ident) => self.0.define(ident.clone(), RefCell::new(None)).unwrap(),
            LitEq(_) => {}
            Unpack(unpack) => {
                for subpat in unpack.values() {
                    self.declare(subpat);
                }
            }
        }
    }

    pub fn bind_pattern(&self, pattern: &Pattern, value: &ValRef) -> Result<(), BindFailure> {
        use Pattern::*;

        let into_bf = |r| BindFailure::new(pattern, value, r);

        match pattern {
            Bind(ident) => self.bind_ident(ident.as_str(), value).map_err(into_bf),
            LitEq(lit) => bind_lit_eq(lit, value).map_err(into_bf),
            Unpack(unpack) => self.bind_unpack(unpack, value),
        }
    }

    /// Return [Result]<[Option]<[ValRef]>, [Unbound]> where `None` indicates the binding is not
    /// declared in this frame. If a binding is declared, but not defined, this is an
    /// [Unfulfilled] error.
    pub fn deref(&self, ident: &IdentRef) -> Result<Option<ValRef>, Unbound> {
        self.0
            .get(ident)
            .map(|rcell| {
                let optval: Option<ValRef> = rcell.borrow().clone();
                optval.ok_or_else(|| Unfulfilled.make(ident))
            })
            .transpose()
    }

    fn bind_ident(&self, ident: &IdentRef, value: &ValRef) -> Result<(), BindFailureReason> {
        let cell = self
            .0
            .get(ident)
            .unwrap_or_else(|| panic!("attempt to bind undeclared binding: {:?}", ident));

        if cell.borrow_mut().replace(value.clone()).is_none() {
            Ok(())
        } else {
            panic!("redefinition of {:?}", ident);
        }
    }

    fn bind_unpack(&self, unpack: &UnpackPattern, value: &ValRef) -> Result<(), BindFailure> {
        self.bind_unpack_inner(unpack, value).map_err(|e| match e {
            Failure(bf) => bf,
            Reason(r) => BindFailure::new(&Pattern::Unpack(unpack.clone()), value, r),
        })
    }

    fn bind_unpack_inner(
        &self,
        unpack: &UnpackPattern,
        value: &ValRef,
    ) -> Result<(), InnerFailure> {
        use BindFailureReason::MissingAttr;

        let srcattrs: &Attrs = value.coerce()?;
        check_unexpected_source_attrs(unpack, srcattrs)?;

        for (ident, pat) in unpack.iter() {
            let v = srcattrs
                .get(ident)
                .ok_or_else(|| MissingAttr(ident.to_string()))?;

            self.bind_pattern(pat, v)?;
        }

        Ok(())
    }
}

fn bind_lit_eq(lit: &Literal, value: &ValRef) -> Result<(), BindFailureReason> {
    use Literal::Num;

    match lit {
        Num(expected) if value.coerce::<f64>()? == expected => Ok(()),
        _ => Err(BindFailureReason::LitNotEqual),
    }
}

// Used to propagate inner BindFailures:
enum InnerFailure {
    Failure(BindFailure),
    Reason(BindFailureReason),
}
use InnerFailure::*;

impl From<BindFailure> for InnerFailure {
    fn from(bf: BindFailure) -> InnerFailure {
        Failure(bf)
    }
}

impl<T> From<T> for InnerFailure
where
    BindFailureReason: From<T>,
{
    fn from(x: T) -> InnerFailure {
        Reason(x.into())
    }
}
fn check_unexpected_source_attrs(
    unpack: &UnpackPattern,
    srcattrs: &Attrs,
) -> Result<(), BindFailureReason> {
    use std::collections::BTreeSet;
    use BindFailureReason::UnexpectedAttrs;

    let unpacknames: BTreeSet<_> = unpack.keys().collect();
    let srcnames: BTreeSet<_> = srcattrs.keys().collect();
    let unexpected: Vec<String> = srcnames
        .difference(&unpacknames)
        .map(|s| s.to_string())
        .collect();

    if unexpected.is_empty() {
        Ok(())
    } else {
        Err(UnexpectedAttrs(unexpected))
    }
}
