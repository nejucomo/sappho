pub trait Embed<T> {
    fn embed(thing: T) -> Self;
}

impl<T> Embed<T> for T {
    fn embed(thing: T) -> Self {
        thing
    }
}

impl<T> Embed<T> for Option<T> {
    fn embed(thing: T) -> Self {
        Some(thing)
    }
}

impl<T, E> Embed<T> for Result<T, E> {
    fn embed(thing: T) -> Self {
        Ok(thing)
    }
}
