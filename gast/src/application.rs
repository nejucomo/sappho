use sappho_unparse::{Stream, Unparse};

/// Function application, ie `f x`.
#[derive(Clone, Debug, PartialEq, derive_new::new)]
pub struct ApplicationExpr<Expr> {
    /// The target of application, ie `f` in `f x`.
    pub target: Box<Expr>,

    /// The argument of application, ie `x` in `f x`.
    pub argument: Box<Expr>,
}

impl<X> ApplicationExpr<X> {
    pub fn transform_into<Y>(self) -> ApplicationExpr<Y>
    where
        Y: From<X>,
    {
        ApplicationExpr {
            target: Box::new(Y::from(*self.target)),
            argument: Box::new(Y::from(*self.argument)),
        }
    }
}

impl<Expr> Unparse for ApplicationExpr<Expr>
where
    Expr: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        use sappho_unparse::Break::{Opt, OptSpace};

        s.write_str_break("(", Opt);
        let mut subs = Stream::new();
        self.target.unparse(&mut subs);
        subs.add_break(OptSpace);
        self.argument.unparse(&mut subs);
        s.add_substream(subs);
        s.add_break(Opt);
        s.write_str(")");
    }
}
