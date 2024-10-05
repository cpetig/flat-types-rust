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
pub use writer::{Assign, Creator, Fill};

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
        /// Make intention explicit and distinguish from buffer bytes
        type SmallIndex = u8;
        let str = View::<String<SmallIndex>>::new(&buffer[16..]);
        assert_eq!(format!("{str:?}"), "test");
        let str = View::<String<SmallIndex>>::new(&buffer[6..]);
        assert_eq!(format!("{str:?}"), "A");
        let str = View::<String<SmallIndex>>::new(&buffer[10..]);
        assert_eq!(format!("{str:?}"), "CC");
        let vec = View::<Vec<String<SmallIndex>, SmallIndex>>::new(&buffer[2..]);
        assert_eq!(format!("{vec:?}"), "[A, B, CC]");
        let vec = View::<Vec<Vec<String<SmallIndex>, SmallIndex>, SmallIndex>>::new(&buffer);
        assert_eq!(format!("{vec:?}"), "[[A, B, CC], [test]]");

        let mut writebuffer = [0u8; 256];
        let writer = Creator::<String<SmallIndex>>::new(&mut writebuffer);
        let view = writer.set("test").expect("write ok");
        assert_eq!(format!("{view:?}"), "test");

        let mut writer = Creator::<Vec<String<SmallIndex>, SmallIndex>>::new(&mut writebuffer);
        writer.allocate(2).expect("root alloc");
        writer.push(|w| w.set("hello")).expect("element created");
        writer.push(|w| w.set("world")).expect("element created");
        let view = writer.finish().expect("ready");
        //        dbg!(&view.buffer);
        assert_eq!(format!("{view:?}"), "[hello, world]");
    }
}
