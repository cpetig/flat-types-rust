use super::{IndexType, View};
use std::fmt::Debug;
use std::marker::PhantomData;

/// Vector inside a flat buffer
#[derive(Copy, Clone)]
pub struct Vec<T, IDX: Copy> {
    delta: IDX,
    length: IDX,
    phantom: PhantomData<T>,
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
