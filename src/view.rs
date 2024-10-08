use std::marker::PhantomData;

/// Read access to a particular type inside a flat buffer
pub struct View<'a, T: Copy> {
    pub(crate) buffer: &'a [u8],
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
