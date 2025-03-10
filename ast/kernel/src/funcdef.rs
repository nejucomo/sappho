use sappho_unparse::{Stream, Unparse};

use crate::Recursion;

#[derive(Debug, derive_new::new)]
pub struct FuncDef<R>
where
    R: Unparse,
{
    binding: (),
    body: Recursion<R>,
}

impl<R> Unparse for FuncDef<R>
where
    R: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        dbg!(self.binding);

        s.write("fn ");
        // s.write(&self.binding);
        s.write("FIXME: implement patterns");
        s.write(" -> ");
        s.write(&self.body);

        todo!("implement patterns");
    }
}
