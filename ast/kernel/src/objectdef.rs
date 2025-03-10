use sappho_attrs::Attrs;
use sappho_unparse::Unparse;

#[derive(Debug, derive_more::From)]
pub struct ObjectDef<R>
where
    R: Unparse,
{
    // proc: Option<ProcDef<R>>,
    // query: Option<QueryDef<R>>,
    attrs: Attrs<R>,
}

impl<R> Unparse for ObjectDef<R>
where
    R: Unparse,
{
    fn unparse_into(&self, s: &mut sappho_unparse::Stream) {
        s.write(&self.attrs);
    }
}
