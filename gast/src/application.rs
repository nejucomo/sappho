use sappho_fmtutil::{DisplayDepth, FmtResult, Formatter};

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

impl<Expr> DisplayDepth for ApplicationExpr<Expr>
where
    Expr: DisplayDepth,
{
    fn fmt_depth(&self, f: &mut Formatter, depth: usize) -> FmtResult {
        use sappho_fmtutil::indent;

        writeln!(f, "(")?;
        indent(f, depth + 1)?;
        self.target.fmt_depth(f, depth + 1)?;
        writeln!(f)?;
        indent(f, depth + 1)?;
        self.argument.fmt_depth(f, depth + 1)?;
        writeln!(f)?;
        indent(f, depth)?;
        write!(f, ")")?;
        Ok(())
    }
}
