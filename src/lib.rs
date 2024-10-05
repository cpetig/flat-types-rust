use std::{fmt::Debug, marker::PhantomData};
mod view;
mod writer;
mod error;
mod vec; mod string; mod index_type;

pub use view::View;
pub use writer::{Writer, Assign, Context};
pub use error::Error;
pub use vec::Vec;
use index_type::IndexType;
pub use string::String;
//use writer::{};

impl<'a, IDX: IndexType + Copy> Debug for View<'a, String<IDX>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let start = IDX::read(self.buffer);
        let len = IDX::read(&self.buffer[std::mem::size_of::<IDX>()..]);
        let view = &self.buffer[start..(start + len)];
        let string = std::str::from_utf8(view)
            .or_else(|e| std::str::from_utf8(&view[..e.valid_up_to()]))
            .unwrap();
        f.write_str(string)
    }
}

impl<'a, T: Copy, IDX: IndexType + Copy> Debug for View<'a, Vec<T, IDX>>
where
    View<'a, T>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let start = IDX::read(self.buffer);
        let len = IDX::read(&self.buffer[std::mem::size_of::<IDX>()..]);
        let elems = &self.buffer[start..];
        let elemsize = std::mem::size_of::<T>();
        let mut lst = f.debug_list();
        for i in 0..len {
            let elem = &elems[i * elemsize..];
            lst.entry(&View::<T>::new(elem));
        }
        lst.finish()
    }
}

// impl<'a, T: Copy, U: Assignable<T>> Writer<'a,T> {
//         pub fn set(&mut self, value: U) -> View<'a, T> { todo!()}
// }

impl<'a, IDX: IndexType + Copy> Assign<'a, &str, String<IDX>> for Writer<'a, String<IDX>> {
    fn set(mut self, value: &str) -> Result<View<'a, String<IDX>>, Error> {
        if self.current_end != 0 {
            return Err(Error::BufferBusy);
        }
        let data_pos = 2 * core::mem::size_of::<IDX>();
        let len = value.len();
        self.current_end += data_pos + len;
        IDX::write(self.buffer, data_pos)?;
        IDX::write(&mut self.buffer[core::mem::size_of::<IDX>()..], len)?;
        self.buffer[data_pos..data_pos + len].copy_from_slice(value.as_bytes());
        Ok(View::new(&self.buffer[0..self.current_end]))
    }
}

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
