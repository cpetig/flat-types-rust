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

/// Creation access to a particular type inside a flat buffer
pub struct Creator<'a, T> {
    pub(crate) buffer: &'a mut [u8],
    phantom: PhantomData<T>,
    pub(crate) current_end: usize,
}

pub trait Assign<'a, T, U: Copy> {
    fn set(self, value: T) -> Result<View<'a, U>, Error>;
}

impl<'a, T: Copy> Creator<'a, T> {
    pub fn new(ctx: Context<'a>) -> Self {
        Self {
            buffer: ctx.buffer,
            phantom: PhantomData,
            current_end: 0,
        }
    }
}
