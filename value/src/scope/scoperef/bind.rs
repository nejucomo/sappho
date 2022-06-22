use crate::{Attrs, BindFailure, BindFailureReason, ValRef};
use sappho_east::{Identifier, Literal, Pattern, UnpackPattern};

/// Attempt to bind `value` to `pattern` into new [Attrs] on success.
pub(super) fn bind_attrs(pattern: &Pattern, value: &ValRef) -> Result<Attrs, BindFailure> {
    let mut attrs = Attrs::default();
    bind_to_attrs(&mut attrs, pattern, value)?;
    Ok(attrs)
}

fn bind_to_attrs(attrs: &mut Attrs, pattern: &Pattern, value: &ValRef) -> Result<(), BindFailure> {
    use Pattern::*;

    let into_bf = |r| BindFailure::new(pattern, value, r);

    match pattern {
        Bind(ident) => bind_bind(attrs, ident, value).map_err(into_bf),
        LitEq(lit) => bind_lit_eq(lit, value).map_err(into_bf),
        Unpack(unpack) => bind_unpack(attrs, unpack, value),
    }
}

fn bind_bind(
    attrs: &mut Attrs,
    ident: &Identifier,
    value: &ValRef,
) -> Result<(), BindFailureReason> {
    // BUG: unwrap `RedefinitionError` which should be detected statically prior to binding
    // evaluation.
    attrs.define(ident.clone(), value.clone()).unwrap();
    Ok(())
}

fn bind_lit_eq(lit: &Literal, value: &ValRef) -> Result<(), BindFailureReason> {
    use Literal::Num;

    match lit {
        Num(expected) if value.coerce::<f64>()? == expected => Ok(()),
        _ => Err(BindFailureReason::LitNotEqual),
    }
}

fn bind_unpack(
    newscope: &mut Attrs,
    unpack: &UnpackPattern,
    value: &ValRef,
) -> Result<(), BindFailure> {
    bind_unpack_inner(newscope, unpack, value).map_err(|e| match e {
        Failure(bf) => bf,
        Reason(r) => BindFailure::new(&Pattern::Unpack(unpack.clone()), value, r),
    })
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

fn bind_unpack_inner(
    newscope: &mut Attrs,
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

        bind_to_attrs(newscope, pat, v)?;
    }

    Ok(())
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
