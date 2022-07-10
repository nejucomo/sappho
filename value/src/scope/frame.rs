mod bindfailure;

use crate::{Attrs, Unbound, ValRef};
use sappho_east::{Literal, Pattern, UnpackPattern};
use sappho_identmap::{IdentMap, IdentRef};

pub use self::bindfailure::{BindFailure, BindFailureReason};

/// A `Frame` maps in-scope bindings to [Option]<[ValRef]> where `None` indicates a
/// forward-reference is not yet fulfilled, while `Some(v)` provides a defined value.
#[derive(Debug, Default, derive_more::From)]
pub struct Frame(IdentMap<Option<ValRef>>);

impl Frame {
    pub fn from_pattern_binding(pattern: &Pattern, value: &ValRef) -> Result<Frame, BindFailure> {
        let mut frame = Frame::default();
        frame.bind_pattern(pattern, value)?;
        Ok(frame)
    }

    /// Return [Result]<[Option]<[ValRef]>, [Unbound]> where `None` indicates the binding is not
    /// declared in this frame. If a binding is declared, but not defined, this is an
    /// [Unbound::Unfulfilled] error.
    pub fn deref(&self, ident: &IdentRef) -> Result<Option<ValRef>, Unbound> {
        use crate::UnboundKind::Unfulfilled;

        self.0
            .get(ident)
            // Clone the entry to `Option<ValRef>`, if present:
            .cloned()
            // If there is a `None`, the binding is unfulfilled:
            .map(|optval| optval.ok_or_else(|| Unfulfilled.make(ident)))
            // Transpose so that a missing binding becomes `Ok(None)`:
            .transpose()
    }

    fn bind_pattern(&mut self, pattern: &Pattern, value: &ValRef) -> Result<(), BindFailure> {
        use Pattern::*;

        let into_bf = |r| BindFailure::new(pattern, value, r);

        match pattern {
            Bind(ident) => self.bind_ident(ident.as_str(), value).map_err(into_bf),
            LitEq(lit) => bind_lit_eq(lit, value).map_err(into_bf),
            Unpack(unpack) => self.bind_unpack(unpack, value),
        }
    }

    fn bind_ident(&mut self, ident: &IdentRef, value: &ValRef) -> Result<(), BindFailureReason> {
        // BUG: unwrap `RedefinitionError` which should be detected statically prior to binding
        // evaluation.
        self.0
            .define(ident.to_string(), Some(value.clone()))
            .unwrap();
        Ok(())
    }

    fn bind_unpack(&mut self, unpack: &UnpackPattern, value: &ValRef) -> Result<(), BindFailure> {
        self.bind_unpack_inner(unpack, value).map_err(|e| match e {
            Failure(bf) => bf,
            Reason(r) => BindFailure::new(&Pattern::Unpack(unpack.clone()), value, r),
        })
    }

    fn bind_unpack_inner(
        &mut self,
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
