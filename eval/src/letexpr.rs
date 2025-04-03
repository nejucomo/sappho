use sappho_east::{Let, LetClause, Wise};
use sappho_effect::Effect;
use sappho_pattern::Pattern;
use sappho_value::{Locals, Scope, Value};

use crate::evco::{Continuation, Eval};
use crate::scoped::Scoped;
use crate::step::Step::{self, Continue};

type ClauseIter<FX> = <Vec<LetClause<FX>> as IntoIterator>::IntoIter;

/// # BUGs
///
/// - BUG: this evaluates in the containing lexical scope, which prevents sequential value composition and (single or mutual) recursion.
#[derive(Debug)]
pub(crate) struct LetCont<FX>
where
    FX: Effect,
{
    binding: Pattern,
    state: State<FX>,
}

#[derive(Debug)]
struct State<FX>
where
    FX: Effect,
{
    scope: Scope,
    locals: Locals,
    clauses: ClauseIter<FX>,
    inner: Wise<FX>,
}

impl<FX> Eval<FX> for Scoped<Let<FX>>
where
    FX: Effect,
{
    type Continuation = LetCont<FX>;

    fn eval_step(self) -> Step<Scoped<Wise<FX>>, Self::Continuation> {
        let state = State::from(self);
        state.step()
    }
}

impl<FX> Continuation<FX> for LetCont<FX>
where
    FX: Effect,
{
    fn eval_from_value(mut self, v: Value) -> Step<Scoped<Wise<FX>>, Self> {
        self.state.locals.bind(self.binding, v).unwrap();
        self.state.step()
    }
}

impl<FX> From<Scoped<Let<FX>>> for State<FX>
where
    FX: Effect,
{
    fn from(sclet: Scoped<Let<FX>>) -> Self {
        State {
            scope: sclet.scope,
            locals: Locals::default(),
            clauses: sclet.node.clauses.into_iter(),
            inner: sclet.node.inner.unwrap(),
        }
    }
}

impl<FX> State<FX>
where
    FX: Effect,
{
    fn step(mut self) -> Step<Scoped<Wise<FX>>, LetCont<FX>> {
        if let Some(LetClause {
            binding,
            definition,
        }) = self.clauses.next()
        {
            Continue(
                self.scope.clone().wrap(definition.unwrap()),
                Some(LetCont {
                    binding,
                    state: self,
                }),
            )
        } else {
            Continue(self.scope.wrap(self.inner), None)
        }
    }
}
