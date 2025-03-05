use crate::error::BareError;
use chumsky::Parser;
use sappho_listform::ListForm;

pub(crate) fn list_form<PX, PT, X, T>(
    item: PX,
    tail: PT,
) -> impl Parser<char, ListForm<X, T>, Error = BareError>
where
    PX: Parser<char, X, Error = BareError>,
    PT: Parser<char, T, Error = BareError> + Clone,
{
    use crate::delimited::delimited;
    use crate::space::ws;
    use chumsky::primitive::just;

    let tailmatch = || just("..").ignore_then(tail.clone());
    let nonempty_body = item.separated_by(just(',').then(ws().or_not())).at_least(1);

    let nonempty_opt_tail = nonempty_body
        .then(
            just(',')
                .then_ignore(ws())
                .ignore_then(tailmatch())
                .or_not(),
        )
        .map(|(pats, opttail)| ListForm::new(pats, opttail));

    delimited(
        '[',
        tailmatch()
            .map(|t| ListForm::new([], Some(t)))
            .or(nonempty_opt_tail)
            .or_not()
            .map(|opt| opt.unwrap_or_else(|| ListForm::new([], None))),
        ']',
    )
}
