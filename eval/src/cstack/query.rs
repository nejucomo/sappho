use derive_more::From;
use sappho_east::Wise;
use sappho_effect::{PureEffect, QueryEffect};
use sappho_value::Value;

use crate::continuation::Continuation;
use crate::evfx::HasCStack;
use crate::expr::ExprCont;
use crate::scoped::Scoped;
use crate::step::Step;

use self::QueryTC::*;

impl HasCStack for QueryEffect {
    type MultiExpr = QueryMX;
    type MultiCont = QueryTC;
    type Stack;
}

#[derive(Debug, From)]
pub(crate) enum QueryMX {
    MXQ(Wise<QueryEffect>),
    MXP(Wise<PureEffect>),
}

#[derive(Debug, From)]
pub(crate) enum QueryTC {
    TCQ(ExprCont<QueryEffect>),
    TCP(ExprCont<PureEffect>),
}

impl Continuation<QueryEffect> for QueryTC {
    fn eval_from_value(self, v: Value) -> Step<Scoped<Wise<QueryEffect>>, Self> {
        match self {
            Q(x) => x.eval_from_value(v),
            P(x) => x.eval_from_value(v),
        }
    }
}

impl Continuation<QueryEffect> for QueryTC {
    fn eval_from_value(self, v: Value) -> Step<Scoped<Wise<QueryEffect>>, Self> {
        match self {
            Q(x) => x.eval_from_value(v),
            P(x) => x.eval_from_value(v),
        }
    }
}
