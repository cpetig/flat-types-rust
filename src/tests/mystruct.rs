use crate::{Creator, Error, String, Vec, View};
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
            .field(
                "b",
                &View::<String<SmallIndex>>::new(&self.buffer[offset_b..]),
            )
            .finish()
    }
}

// writing is more complex
pub trait FillMyStruct<'a: 'short, 'short> {
    /// in place construct elements
    fn set_a<
        'slf: 'short,
        F: Fn(
            Creator<'short, Vec<u32, SmallIndex>>,
        ) -> Result<View<'short, Vec<u32, SmallIndex>>, Error>,
    >(
        &'slf mut self,
        f: F,
    ) -> Result<(), Error>;
    fn set_b<
        'slf: 'short,
        F: Fn(Creator<'short, String<SmallIndex>>) -> Result<View<'short, String<SmallIndex>>, Error>,
    >(
        &'slf mut self,
        f: F,
    ) -> Result<(), Error>;
    /// finalize
    fn finish(self) -> Result<View<'a, MyStruct>, Error>;
}

impl<'a: 'short, 'short> FillMyStruct<'a, 'short> for Creator<'a, MyStruct> {
    fn finish(self) -> Result<View<'a, MyStruct>, crate::Error> {
        if self.valid_elements != 3 {
            return Err(Error::AllocationTooLarge(self.valid_elements, 3));
        }
        Ok(View::new(&self.buffer[..self.current_end]))
    }

    fn set_a<
        'slf: 'short,
        F: Fn(
            Creator<'short, Vec<u32, SmallIndex>>,
        ) -> Result<View<'short, Vec<u32, SmallIndex>>, crate::Error>,
    >(
        &'slf mut self,
        f: F,
    ) -> Result<(), crate::Error> {
        if (self.valid_elements & 1) != 0 {
            return Err(Error::BufferBusy);
        }
        let start = 0;
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

    fn set_b<
        'slf: 'short,
        F: Fn(
            Creator<'short, String<SmallIndex>>,
        ) -> Result<View<'short, String<SmallIndex>>, crate::Error>,
    >(
        &'slf mut self,
        f: F,
    ) -> Result<(), crate::Error> {
        if (self.valid_elements & 2) != 0 {
            return Err(Error::BufferBusy);
        }
        let start = core::mem::size_of::<Vec<u32, SmallIndex>>();
        let sub_allocation = self.current_end - start;
        let sub_creator = Creator::new_filled(&mut self.buffer[start..], sub_allocation);
        let view = f(sub_creator)?;
        if view.buffer.len() != sub_allocation {
            assert!(self.current_end < view.buffer.len() + start);
            assert!(sub_allocation < view.buffer.len());
            self.current_end = view.buffer.len() + start;
        }
        self.valid_elements += 2;
        Ok(())
    }
}
