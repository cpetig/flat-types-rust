use super::{Error, View};
use std::marker::PhantomData;

// pub struct Context<'a> {
//     buffer: &'a mut [u8],
// }

// impl<'a> Context<'a> {
//     pub fn new(buffer: &'a mut [u8]) -> Self {
//         Self { buffer }
//     }
// }

/// Creation access to a particular type inside a flat buffer
pub struct Creator<'a, T> {
    pub(crate) buffer: &'a mut [u8],
    phantom: PhantomData<T>,
    pub(crate) current_end: usize,
    pub(crate) valid_elements: usize,
}

pub trait Assign<'a, T, U: Copy> {
    fn set(self, value: T) -> Result<View<'a, U>, Error>;
}

pub trait Fill<'a: 'short, 'short, T: Copy, SUB: Copy> {
    // 🤔 this could also consume self and return an object which you call push and drop on?
    /// allocate elements in vector (first step)
    fn allocate(&mut self, size: usize) -> Result<(), Error>;
    // do we correctly handle pushing a vec?
    /// in place construct another element
    fn push<'slf: 'short, F: Fn(Creator<'short, SUB>) -> Result<View<'short, SUB>, Error>>(
        &'slf mut self,
        f: F,
    ) -> Result<(), Error>;
    /// finalize
    fn finish(self) -> Result<View<'a, T>, Error>;
}

impl<'a, T: Copy> Creator<'a, T> {
    pub fn new(buffer: &'a mut [u8]) -> Self {
        Self::new_filled(buffer, core::mem::size_of::<T>())
    }
    pub fn new_filled(buffer: &'a mut [u8], current_end: usize) -> Self {
        Self {
            buffer,
            phantom: PhantomData,
            current_end,
            valid_elements: 0,
        }
    }
}
