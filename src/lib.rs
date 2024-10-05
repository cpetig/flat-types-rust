use std::fmt::Debug;
mod error;
mod index_type;
mod string;
mod vec;
mod view;
mod writer;

pub use error::Error;
use index_type::IndexType;
pub use string::String;
pub use vec::Vec;
pub use view::View;
pub use writer::{Assign, Context, Writer};
//use writer::{};

// impl<'a, T: Copy, U: Assignable<T>> Writer<'a,T> {
//         pub fn set(&mut self, value: U) -> View<'a, T> { todo!()}
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let buffer: [u8; 22] = [
            /* toplevel vec */ 2, 2, /* 1st level entries(2 vecs) */ 4, 3, 12, 1,
            /* 2nd level entries (3 strings) */ 6, 1, 5, 1, 4, 2, /* String */ b'A',
            /* String */ b'B', /* String */ b'C', b'C',
            /* 2nd level entries (1 string) */ 2, 4, /* String */ b't', b'e', b's', b't',
        ];
        let str = View::<String<u8>>::new(&buffer[16..]);
        assert_eq!(format!("{str:?}"), "test");
        let str = View::<String<u8>>::new(&buffer[6..]);
        assert_eq!(format!("{str:?}"), "A");
        let str = View::<String<u8>>::new(&buffer[10..]);
        assert_eq!(format!("{str:?}"), "CC");
        let vec = View::<Vec<String<u8>, u8>>::new(&buffer[2..]);
        assert_eq!(format!("{vec:?}"), "[A, B, CC]");
        let vec = View::<Vec<Vec<String<u8>, u8>, u8>>::new(&buffer);
        assert_eq!(format!("{vec:?}"), "[[A, B, CC], [test]]");

        let mut writebuffer = [0u8; 256];
        let ctx = Context::new(&mut writebuffer);
        let writer = Writer::<String<u8>>::new(ctx);
        let view = writer.set("test").expect("write ok");
        assert_eq!(format!("{view:?}"), "test");
    }
}
