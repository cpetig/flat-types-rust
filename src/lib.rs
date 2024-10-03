use std::{fmt::Debug, marker::PhantomData};

// #[derive(Copy, Clone)]
// struct Slice<const N: usize, T: Copy> {
//     slice: [T; N],
// }

#[derive(Copy, Clone)]
struct Vec<T, IDX: Copy> {
    delta: IDX,
    length: IDX,
    phantom: PhantomData<T>,
}

// #[derive(Copy, Clone)]
// struct Str<const N: usize> {
//     slice: [u8; N],
// }

#[derive(Copy, Clone)]
struct String<IDX> {
    delta: IDX,
    length: IDX,
}

struct View<'a, T: Copy, IDX: Copy> {
    buffer: &'a [u8],
    phantom: PhantomData<(T, IDX)>,
}

impl<'a, T: Copy, IDX: Copy> View<'a, T, IDX> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self {
            buffer,
            phantom: PhantomData,
        }
    }
}

trait ReadIndex: Copy {
    //+ Into<usize> {
    fn read(buffer: &[u8]) -> usize;
}

impl ReadIndex for u8 {
    fn read(buffer: &[u8]) -> usize {
        buffer[0].into()
    }
}

impl<'a, IDX: ReadIndex> Debug for View<'a, String<IDX>, IDX> {
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

impl<'a, T: Copy, IDX: ReadIndex> Debug for View<'a, Vec<T, IDX>, IDX>
where
    View<'a, T, IDX>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let start = IDX::read(self.buffer);
        let len = IDX::read(&self.buffer[std::mem::size_of::<IDX>()..]);
        let elems = &self.buffer[start..];
        let elemsize = std::mem::size_of::<T>();
        let mut lst = f.debug_list();
        for i in 0..len {
            let elem = &elems[i * elemsize..];
            lst.entry(&View::<T, IDX>::new(elem));
        }
        lst.finish()
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
        let str = View::<String<u8>, u8>::new(&buffer[16..]);
        assert_eq!(format!("{str:?}"), "test");
        let str = View::<String<u8>, u8>::new(&buffer[6..]);
        assert_eq!(format!("{str:?}"), "A");
        let str = View::<String<u8>, u8>::new(&buffer[10..]);
        assert_eq!(format!("{str:?}"), "CC");
        let vec = View::<Vec<String<u8>, u8>, u8>::new(&buffer[2..]);
        assert_eq!(format!("{vec:?}"), "[A, B, CC]");
        let vec = View::<Vec<Vec<String<u8>, u8>, u8>, u8>::new(&buffer);
        assert_eq!(format!("{vec:?}"), "[[A, B, CC], [test]]");
    }
}
