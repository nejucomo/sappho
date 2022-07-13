use crate::Stream;

pub trait Unparse {
    fn unparse(&self) -> Stream {
        let mut s = Stream::new();
        self.unparse_into(&mut s);
        s
    }

    fn unparse_into(&self, s: &mut Stream);
}
