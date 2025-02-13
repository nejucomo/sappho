mod clause;

use sappho_ast_effect::Effect;
use sappho_unparse::{Stream, Unparse};

use crate::{AstProvider, AstTransformInto, BoxExpr};

pub use self::clause::LetClause;

/// A `let` expression for local definitions, ie: `let x = 42; f x`.
#[derive(Debug, derive_new::new)]
pub struct LetExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    /// The let clauses:
    pub clauses: Vec<LetClause<XP, FX>>,

    /// The expression to evaluate with the binding in-scope, ie: `f x` in `let x = 42; f x`.
    pub tail: BoxExpr<XP, FX>,
}

impl<XPD, XPS, FX> AstTransformInto<LetExpr<XPD, FX>> for LetExpr<XPS, FX>
where
    XPD: AstProvider,
    XPS: AstProvider,
    XPS::Pattern: AstTransformInto<XPD::Pattern>,
    XPS::Expr<FX>: AstTransformInto<XPD::Expr<FX>>,
    FX: Effect,
{
    fn ast_transform(self) -> LetExpr<XPD, FX> {
        LetExpr {
            clauses: self
                .clauses
                .into_iter()
                .map(|c| c.ast_transform())
                .collect(),
            tail: self.tail.ast_transform(),
        }
    }
}

impl<XP, FX> Unparse for LetExpr<XP, FX>
where
    XP: AstProvider,
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

impl<XP, FX> Clone for LetExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn clone(&self) -> Self {
        LetExpr::new(self.clauses.clone(), self.tail.clone())
    }
}

impl<XP, FX> PartialEq for LetExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn eq(&self, other: &Self) -> bool {
        self.tail == other.tail && self.clauses == other.clauses
    }
}
