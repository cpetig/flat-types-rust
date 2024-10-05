use super::writer::Fill;
use super::{Creator, IndexType, View};
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

impl<'a, T: Copy, IDX: IndexType + Copy> Fill<'a, Vec<T, IDX>, T> for Creator<'a, Vec<T, IDX>> {
    fn allocate(&mut self, size: usize) -> Result<(), crate::Error> {
        todo!()
    }

    fn finish(self) -> Result<View<'a, Vec<T, IDX>>, crate::Error> {
        todo!()
    }
    
    fn push<F: Fn(Creator<'a, T>) -> Result<View<'a, T>, crate::Error>>(
        &mut self,
        f: F,
    ) -> Result<(), crate::Error> {
        todo!()
    }
}
