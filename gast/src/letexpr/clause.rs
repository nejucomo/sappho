use sappho_unparse::{DisplayDepth, FmtResult, Formatter};

#[derive(Clone, Debug, PartialEq, derive_new::new)]
pub struct LetClause<Pattern, Expr> {
    /// The binding pattern, ie: the first `x` in `let x = 42; f x`.
    pub binding: Pattern,

    /// The expression to bind, ie: `42` in `let x = 42; f x`.
    pub bindexpr: Box<Expr>,
}

impl<P, X> LetClause<P, X> {
    pub fn transform_into<PD, XD>(self) -> LetClause<PD, XD>
    where
        PD: From<P>,
        XD: From<X>,
    {
        LetClause {
            binding: PD::from(self.binding),
            bindexpr: Box::new(XD::from(*self.bindexpr)),
        }
    }
}

impl<P, X> DisplayDepth for LetClause<P, X>
where
    P: DisplayDepth,
    X: DisplayDepth,
{
    fn fmt_depth(&self, f: &mut Formatter, depth: usize) -> FmtResult {
        write!(f, "let ")?;
        self.binding.fmt_depth(f, depth)?;
        write!(f, " = ")?;
        self.bindexpr.fmt_depth(f, depth)?;
        Ok(())
    }
}
