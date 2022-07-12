mod clause;

use sappho_fmtutil::{DisplayDepth, FmtResult, Formatter};

pub use self::clause::MatchClause;

/// A `match` expression, ie: `match x { 3 -> 0, y -> y }`.
#[derive(Clone, Debug, PartialEq, derive_new::new)]
pub struct MatchExpr<Pattern, Expr> {
    /// The match target, ie: `x` in `match x { 3 -> 0, y -> y }`.
    pub target: Box<Expr>,

    /// The match clauses, ie: `3 -> 0` and `y -> y` in `match x { 3 -> 0, y -> y }`.
    pub clauses: Vec<MatchClause<Pattern, Expr>>,
}

impl<P, X> MatchExpr<P, X> {
    pub fn transform_into<PD, XD>(self) -> MatchExpr<PD, XD>
    where
        PD: From<P>,
        XD: From<X>,
    {
        MatchExpr {
            target: Box::new(XD::from(*self.target)),
            clauses: self
                .clauses
                .into_iter()
                .map(|c| c.transform_into())
                .collect(),
        }
    }
}

impl<P, X> DisplayDepth for MatchExpr<P, X>
where
    P: DisplayDepth,
    X: DisplayDepth,
{
    fn fmt_depth(&self, f: &mut Formatter, depth: usize) -> FmtResult {
        use sappho_fmtutil::indent;

        write!(f, "match ")?;
        self.target.fmt_depth(f, depth)?;
        write!(f, " {{")?;
        for clause in &self.clauses {
            indent(f, depth + 1)?;
            clause.fmt_depth(f, depth + 1)?;
            writeln!(f, ",")?;
        }
        indent(f, depth)?;
        write!(f, "}}")?;
        Ok(())
    }
}
