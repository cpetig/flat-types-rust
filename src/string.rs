use super::{Assign, Creator, Error, IndexType, View};
use std::fmt::Debug;

/// String inside a flat buffer
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct String<IDX> {
    delta: IDX,
    length: IDX,
}

impl<'a, IDX: IndexType + Copy> Debug for View<'a, String<IDX>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let start = IDX::read(self.buffer);
        let len = IDX::read(&self.buffer[std::mem::size_of::<IDX>()..]);
        let view = &self.buffer[start..(start + len)];
        let string = std::str::from_utf8(view)
            .or_else(|e| std::str::from_utf8(&view[..e.valid_up_to()]))
            .unwrap();
        f.write_str(string)
    }
}

impl<'a, IDX: IndexType + Copy> Assign<'a, &str, String<IDX>> for Creator<'a, String<IDX>> {
    fn set(mut self, value: &str) -> Result<View<'a, String<IDX>>, Error> {
        let data_pos = self.current_end;
        let len = value.len();
        self.current_end += len;
        IDX::write(self.buffer, data_pos)?;
        IDX::write(&mut self.buffer[core::mem::size_of::<IDX>()..], len)?;
        self.buffer[data_pos..data_pos + len].copy_from_slice(value.as_bytes());
        Ok(View::new(&self.buffer[0..self.current_end]))
    }
}
