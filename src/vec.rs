use super::writer::Fill;
use super::{Creator, Error, IndexType, View};
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

impl<'a: 'short, 'short, T: Copy, IDX: IndexType + Copy> Fill<'a, 'short, Vec<T, IDX>, T>
    for Creator<'a, Vec<T, IDX>>
{
    fn allocate(&mut self, size: usize) -> Result<(), crate::Error> {
        if self.valid_elements != 0 {
            return Err(Error::BufferBusy);
        }
        let data_pos = self.current_end;
        //        2 * core::mem::size_of::<IDX>();
        let elem_size = core::mem::size_of::<T>();
        self.current_end += size * elem_size;
        IDX::write(self.buffer, data_pos)?;
        IDX::write(&mut self.buffer[core::mem::size_of::<IDX>()..], size)?;
        Ok(())
    }

    fn finish(self) -> Result<View<'a, Vec<T, IDX>>, crate::Error> {
        // cache this?
        let alloc_size = IDX::read(&self.buffer[core::mem::size_of::<IDX>()..]);
        if self.valid_elements != alloc_size {
            return Err(Error::AllocationTooLarge(self.valid_elements, alloc_size));
        }
        Ok(View::new(&self.buffer[..self.current_end]))
    }

    fn push<'slf: 'short, F: Fn(Creator<'short, T>) -> Result<View<'short, T>, crate::Error>>(
        &'slf mut self,
        f: F,
    ) -> Result<(), crate::Error> {
        let elem_size = core::mem::size_of::<T>();
        // cache this?
        let alloc_size = IDX::read(&self.buffer[core::mem::size_of::<IDX>()..]);
        let mut start = IDX::read(self.buffer);
        if self.valid_elements >= alloc_size {
            return Err(Error::AllocationTooSmall(alloc_size));
        }
        start += self.valid_elements * elem_size;
        let sub_allocation = self.current_end - start;
        let sub_creator = Creator::new_filled(&mut self.buffer[start..], sub_allocation);
        let view = f(sub_creator)?;
        if view.buffer.len() != sub_allocation {
            assert!(self.current_end < view.buffer.len() + start);
            assert!(sub_allocation < view.buffer.len());
            self.current_end = view.buffer.len() + start;
        }
        self.valid_elements += 1;
        Ok(())
    }
}
