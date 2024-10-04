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

struct View<'a, T: Copy> {
    buffer: &'a [u8],
    phantom: PhantomData<T>,
}

impl<'a, T: Copy> View<'a, T> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self {
            buffer,
            phantom: PhantomData,
        }
    }
}

enum Error {
    TooSmall,
}

trait ReadIndex {
    //+ Into<usize> {
    fn read(buffer: &[u8]) -> usize;
    fn write(buffer: &[u8], value: usize) -> Result<usize, Error>;
}

impl ReadIndex for u8 {
    fn read(buffer: &[u8]) -> usize {
        buffer[0].into()
    }
    
    fn write(buffer: &[u8], value: usize) -> Result<usize, Error> {
        todo!()
    }   
}

impl<'a, IDX: ReadIndex + Copy> Debug for View<'a, String<IDX>> {
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

impl<'a, T: Copy, IDX: ReadIndex + Copy> Debug for View<'a, Vec<T, IDX>>
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

struct Context<'a> {
    buffer: &'a mut [u8],
}

impl<'a> Context<'a> {
    pub fn new(buffer: &'a mut [u8]) -> Self {
        Self{buffer}
    }
}

struct Writer<'a, T> {
    buffer: &'a mut [u8],
    phantom: PhantomData<T>,
}

trait Assign<T> {
    fn set(self, value: T);
}

trait Assignable<T> {
    fn set(obj: &mut T, value: Self);
}

impl<'a, T: Copy> Writer<'a,T> {
    // type Context=Context<'a>;

    pub fn new(ctx: Context<'a>) -> Self {
        todo!()
    }
}

impl<'a, T: Copy, U: Assignable<T>> Writer<'a,T> {
        pub fn set(&mut self, value: U) -> View<'a, T> { todo!()}
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
        let view = writer.set("test");
        assert_eq!(format("{view:?}"), "test");
    }
}
