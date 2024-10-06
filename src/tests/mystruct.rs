use crate::{String, Vec, View};
use std::fmt::Debug;

/// Make intention explicit and distinguish from buffer bytes
pub type SmallIndex = u8;

#[derive(Copy, Clone)]
pub struct MyStruct {
    a: Vec<u32, SmallIndex>,
    b: String<SmallIndex>,
}

impl<'a> Debug for View<'a, MyStruct> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let offset_b = core::mem::size_of::<Vec<u32, SmallIndex>>();
        f.debug_struct("MyStruct")
            .field("a", &View::<Vec<u32, SmallIndex>>::new(self.buffer))
            .field("b", &View::<String<SmallIndex>>::new(&self.buffer[offset_b..]))
            .finish()
    }
}
