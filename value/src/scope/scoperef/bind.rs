use crate::{Attrs, ValRef};
use sappho_east::{Literal, Pattern};

/// Attempt to bind `value` to `pattern` into new [Attrs] on success.
pub(super) fn bind_attrs(pattern: &Pattern, value: &ValRef) -> Option<Attrs> {
    let mut attrs = Attrs::default();
    if bind_to_attrs(&mut attrs, pattern, value) {
        Some(attrs)
    } else {
        None
    }
}

fn bind_to_attrs(attrs: &mut Attrs, pattern: &Pattern, value: &ValRef) -> bool {
    use crate::Coerce;
    use Literal::Num;
    use Pattern::*;

    match pattern {
        Bind(ident) => {
            // BUG: unwrap `RedefinitionError` which should be detected statically prior to binding
            // evaluation.
            attrs.define(ident.clone(), value.clone()).unwrap();
            true
        }
        LitEq(Num(expected)) => f64::coerce_from_value(value) == Some(expected),
    }
}
