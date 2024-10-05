use super::{Error, View};
use std::marker::PhantomData;

pub struct Context<'a> {
    buffer: &'a mut [u8],
}

impl<'a> Context<'a> {
    pub fn new(buffer: &'a mut [u8]) -> Self {
        Self { buffer }
    }
}

pub struct Writer<'a, T> {
    pub(crate) buffer: &'a mut [u8],
    phantom: PhantomData<T>,
    pub(crate) current_end: usize,
}

pub trait Assign<'a, T, U: Copy> {
    fn set(self, value: T) -> Result<View<'a, U>, Error>;
}

impl<'a, T: Copy> Writer<'a, T> {
    pub fn new(ctx: Context<'a>) -> Self {
        Self {
            buffer: ctx.buffer,
            phantom: PhantomData,
            current_end: 0,
        }
    }
}
