mod baseimpls;
mod exprimpls;
mod fqpimpls;
mod listimpls;
mod lmalimpls;
mod objimpls;
mod patimpls;
mod rcidimpls;
mod stmtimpls;

use either::Either;

pub(crate) trait TransformInto<T> {
    fn transform(self) -> T;
}

pub(crate) trait TryTransformInto<T>: Sized {
    fn try_transform(self) -> Either<T, Self>;
}
