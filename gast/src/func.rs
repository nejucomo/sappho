use sappho_unparse::{Stream, Unparse};

/// A function definition expression, ie `fn x -> x`.
#[derive(Clone, Debug, PartialEq, derive_new::new)]
pub struct FuncDef<Pattern, PureExpr> {
    /// The binding pattern, ie the initial `x` in `fn x -> x`.
    pub binding: Pattern,

    /// The body, ie the final `x` in `fn x -> x`.
    pub body: Box<PureExpr>,
}

impl<P, X> FuncDef<P, X> {
    pub fn transform_into<PD, XD>(self) -> FuncDef<PD, XD>
    where
        PD: From<P>,
        XD: From<X>,
    {
        FuncDef {
            binding: PD::from(self.binding),
            body: Box::new(XD::from(*self.body)),
        }
    }
}

impl<P, X> Unparse for FuncDef<P, X>
where
    P: Unparse,
    X: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        s.write_str("fn ");
        self.binding.unparse(s);
        s.write_str(" -> ");
        self.body.unparse(s);
    }
}
