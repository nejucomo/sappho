use either::Either::{self, Left, Right};
use sappho_ast::Identifier;

pub(crate) fn reduce_listform<LF, TF, TI, XF, XI, MF, R>(
    listform: LF,
    tail_default: R,
    f_tail: TF,
    f_elem: XF,
    f_idmap: MF,
) -> R
where
    TI: std::fmt::Debug,
    XI: std::fmt::Debug,
    R: std::fmt::Debug,
    LF: IntoIterator<Item = Either<XI, TI>>,
    <LF as IntoIterator>::IntoIter: DoubleEndedIterator,
    TF: Fn(TI) -> R,
    XF: Fn(XI) -> R,
    MF: Fn([(Identifier, R); 2]) -> R,
{
    listform
        .into_iter()
        .rev()
        .fold(tail_default, |tail, head| match dbg!(head) {
            Right(x) => {
                dbg!(&tail);
                dbg!(f_tail(x))
            }
            Left(x) => f_idmap([("head".to_string(), f_elem(x)), ("tail".to_string(), tail)]),
        })
}
