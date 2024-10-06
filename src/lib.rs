mod error;
mod index_type;
mod integers;
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
    mod mystruct;

    use super::*;
    use mystruct::{MyStruct, SmallIndex};

    // type SmallIndex = u8;

    #[test]
    fn reading() {
        let buffer: [u8; 22] = [
            /* toplevel vec */ 2, 2, /* 1st level entries(2 vecs) */ 4, 3, 12, 1,
            /* 2nd level entries (3 strings) */ 6, 1, 5, 1, 4, 2, /* String */ b'A',
            /* String */ b'B', /* String */ b'C', b'C',
            /* 2nd level entries (1 string) */ 2, 4, /* String */ b't', b'e', b's', b't',
        ];
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
    }

    #[test]
    fn writing() {
        let mut writebuffer = [0u8; 256];
        let writer = Creator::<String<SmallIndex>>::new(&mut writebuffer);
        let view = writer.set("test").expect("write ok");
        assert_eq!(format!("{view:?}"), "test");

        let mut writer = Creator::<Vec<String<SmallIndex>, SmallIndex>>::new(&mut writebuffer);
        writer.allocate(2).expect("root alloc");
        writer.push(|w| w.set("hello")).expect("element created");
        writer.push(|w| w.set("world")).expect("element created");
        let view = writer.finish().expect("ready");
        assert_eq!(format!("{view:?}"), "[hello, world]");

        let mut writer =
            Creator::<Vec<Vec<String<SmallIndex>, SmallIndex>, SmallIndex>>::new(&mut writebuffer);
        writer.allocate(2).expect("root alloc");
        writer
            .push(|mut w| {
                w.allocate(3)?;
                w.push(|s| s.set("A"))?;
                w.push(|s| s.set("B"))?;
                w.push(|s| s.set("C"))?;
                w.finish()
            })
            .expect("subvec1 created");
        writer
            .push(|mut w| {
                w.allocate(1)?;
                w.push(|s| s.set("test"))?;
                w.finish()
            })
            .expect("subvec2 created");
        let view = writer.finish().expect("ready");
        dbg!(&view.buffer);
        assert_eq!(format!("{view:?}"), "[[A, B, C], [test]]");
    }

    #[test]
    fn example_struct() {
        #[repr(align(4))]
        struct AlignedArray<const N: usize>([u8; N]);
        let buffer = AlignedArray::<16>([
            4, 2, 10, 4, // Vec<u32>
            42, 0, 0, 0, 21, 205, 91, 7, // String "yolo"
            121, 111, 108, 111,
        ]);

        let str = View::<MyStruct>::new(&buffer.0);
        assert_eq!(
            format!("{str:?}"),
            "MyStruct { a: [42, 123456789], b: yolo }"
        );

        use mystruct::FillMyStruct;
        let mut writebuffer = [0u8; 256];
        let mut writer = Creator::<MyStruct>::new(&mut writebuffer);
        writer.set_a(|mut w| {
            w.allocate(3)?;
            w.push(|s| s.set(1));
            w.push(|s| s.set(2));
            w.push(|s| s.set(3));
            w.finish()
        });
        writer.set_b(|mut w| w.set("hello"));
        let view = writer.finish().expect("ready");
        dbg!(&view.buffer);
        assert_eq!(format!("{view:?}"), "MyStruct { a: [1, 2, 3], b: hello }");
    }
}
