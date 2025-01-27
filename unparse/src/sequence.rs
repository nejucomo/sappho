use crate::{from_fn, Unparse};

pub fn sequence<I, X, S>(items: I, separator: S) -> impl Unparse
where
    I: Clone + IntoIterator<Item = X>,
    X: Unparse,
    S: Copy + Unparse,
{
    from_fn(move |stream| {
        let mut w = false;
        for (i, x) in items.clone().into_iter().enumerate() {
            if i > 0 {
                w |= stream.write(separator)?;
            }
            w |= stream.write(x)?;
        }
        Ok(w)
    })
}
