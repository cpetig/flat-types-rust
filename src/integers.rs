use crate::{Assign, Creator, Error, View};
use std::fmt::Debug;

impl<'a> Debug for View<'a, u32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes: [u8; 4] = (&self.buffer[0..4])
            .try_into()
            .map_err(|_e| std::fmt::Error)?;
        let value = u32::from_le_bytes(bytes);
        f.write_fmt(format_args!("{:?}", value))
    }
}

impl<'a> Assign<'a, u32, u32> for Creator<'a, u32> {
    fn set(mut self, value: u32) -> Result<View<'a, u32>, Error> {
        let bytes = value.to_le_bytes();
        self.buffer[0..4].copy_from_slice(&bytes);
        Ok(View::new(&self.buffer[0..self.current_end]))
    }
}
