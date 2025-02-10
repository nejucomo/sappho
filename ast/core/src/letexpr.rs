mod clause;

use sappho_ast_effect::Effect;
use sappho_unparse::{Stream, Unparse};

use crate::ExprProvider;

pub use self::clause::LetClause;

/// A `let` expression for local definitions, ie: `let x = 42; f x`.
#[derive(Clone, Debug, PartialEq, derive_new::new)]
pub struct LetExpr<XP, FX>
where
    XP: ExprProvider,
    FX: Effect,
{
    /// The let clauses:
    pub clauses: Vec<LetClause<XP, FX>>,

    /// The expression to evaluate with the binding in-scope, ie: `f x` in `let x = 42; f x`.
    pub tail: Box<XP::Expr<FX>>,
}

impl<XP, FX> LetExpr<XP, FX>
where
    XP: ExprProvider,
    FX: Effect,
{
    pub fn transform_into<XPD>(self) -> LetExpr<XPD, FX>
    where
        XPD: ExprProvider,
        XPD::Pattern: From<XP::Pattern>,
        XPD::Expr<FX>: From<XP::Expr<FX>>,
    {
        LetExpr {
            clauses: self
                .clauses
                .into_iter()
                .map(|c| c.transform_into())
                .collect(),
            tail: Box::new(XPD::Expr::from(*self.tail)),
        }
    }
}

impl<XP, FX> Unparse for LetExpr<XP, FX>
where
    XP: ExprProvider,
    FX: Effect,
{
    fn unparse_into(&self, s: &mut Stream) {
        use sappho_unparse::{Brackets::Parens, Break::Mandatory};

        let unparse_clauses = |s: &mut Stream| {
            for (ix, clause) in self.clauses.iter().enumerate() {
                if s.depth() > 0 || ix > 0 {
                    s.write(&Mandatory);
                }
                s.write(clause);
                s.write(";");
            }
            s.write(&Mandatory);
            s.write(&self.tail);
        };

        if s.depth() == 0 {
            unparse_clauses(s);
        } else {
            s.bracketed(Parens, unparse_clauses);
        }
    }
}
